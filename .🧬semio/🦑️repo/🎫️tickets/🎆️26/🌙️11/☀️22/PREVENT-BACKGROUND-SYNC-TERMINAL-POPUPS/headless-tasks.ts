import {existsSync, mkdirSync, readFileSync, rmdirSync, unlinkSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";

const names = ["semio-fleet-store-mount", "semio-fleet-replica", "semio-fleet-compact", "semio-fleet-cache", "semio-fleet-relay"];
const generated = join(import.meta.dirname, "🗑️generated");
const taskRoot = join(process.env.SystemRoot!, "System32", "Tasks");
const quietRoot = process.argv[3] ?? join(process.env.LOCALAPPDATA!, "semio", "fleet", "quiet-tasks");
const helper = join(quietRoot, "quiet.vbs");

function command(program: string, args: string[]): void {
  const result = spawnSync(program, args, {encoding: "utf8", windowsHide: true, timeout: 30_000});
  if (result.error || result.status !== 0) throw new Error(`${program}: ${result.error?.message ?? result.stderr ?? result.status}`);
}

function escapeXml(text: string): string {
  return text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;");
}

function unescapeXml(text: string): string {
  return text.replace(/&(#x[0-9a-f]+|#[0-9]+|amp|lt|gt|quot|apos);/giu, (_, entity: string) => {
    const values: Record<string, string> = {amp: "&", lt: "<", gt: ">", quot: '"', apos: "'"};
    return entity.startsWith("#") ? String.fromCodePoint(entity[1]!.toLowerCase() === "x" ? parseInt(entity.slice(2), 16) : parseInt(entity.slice(1), 10)) : values[entity]!;
  });
}

function headless(xml: string, name: string): {xml: string; line?: string} {
  const actions = [...xml.matchAll(/<Exec>([\s\S]*?)<\/Exec>/gu)];
  if (actions.length !== 1) throw new Error("Expected exactly one Exec action");
  const action = actions[0]![0];
  const match = /<Command>([^<]+)<\/Command>\s*<Arguments>([\s\S]*?)<\/Arguments>/u.exec(action);
  if (!match) throw new Error("Expected an executable and arguments");
  const [, executable, args] = match;
  if (executable!.toLowerCase() === "wscript.exe" && unescapeXml(args!).includes(helper)) return {xml};
  if (!/^[\w.\\:-]+$/u.test(executable!)) throw new Error(`Unsupported executable: ${executable}`);
  const commandFile = join(quietRoot, `${name}.txt`);
  const replacement = `<Command>wscript.exe</Command>\r\n      <Arguments>${escapeXml(`"${helper}" "${commandFile}"`)}</Arguments>`;
  return {xml: xml.replace(action, action.replace(match[0], replacement)), line: `"${executable}" ${unescapeXml(args!)}`};
}

function task(name: string): string {
  return readFileSync(join(taskRoot, name)).toString("utf16le").replace(/^\uFEFF/u, "");
}

function writeXml(path: string, xml: string): void {
  writeFileSync(path, Buffer.concat([Buffer.from([0xff, 0xfe]), Buffer.from(xml, "utf16le")]));
}

function main(): void {
  const mode = process.argv[2];
  if (mode === "request") {
    const repo = Bun.which("repo.exe");
    if (!repo) throw new Error("The repo launcher was not found");
    const quoted = (text: string): string => `'${text.replaceAll("'", "''")}'`;
    mkdirSync(generated, {recursive: true});
    const log = join(generated, "elevated.log");
    const inner = `$env:SEMIO_WORKSPACE_ROOT='C:\\semio'; & ${quoted(repo)} ${quoted(join(import.meta.dirname, "headless-tasks.ts"))} apply ${quoted(quietRoot)} 2>&1 | Out-File -LiteralPath ${quoted(log)} -Encoding utf8; exit $LASTEXITCODE`;
    const arguments_ = `-NoProfile -NonInteractive -EncodedCommand ${Buffer.from(inner, "utf16le").toString("base64")}`;
    const script = `$ErrorActionPreference='Stop'; $p = Start-Process -FilePath 'powershell.exe' -ArgumentList ${quoted(arguments_)} -Verb RunAs -WindowStyle Hidden -Wait -PassThru; exit $p.ExitCode`;
    const result = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-EncodedCommand", Buffer.from(script, "utf16le").toString("base64")], {encoding: "utf8", windowsHide: true, timeout: 180_000});
    if (existsSync(log)) console.log(readFileSync(log, "utf8"));
    if (result.error || result.status !== 0) throw new Error(`Administrator approval or task update failed (${result.status}): ${result.error?.message ?? result.stderr}`);
    console.log("The elevated task update completed");
    return;
  }
  if (mode === "observe") {
    const script = `$ErrorActionPreference='Stop'; $name='semio-fleet-store-mount'; $task=Get-ScheduledTask -TaskName $name; if($task.Actions.Execute -ne 'wscript.exe'){throw 'The mount task is not using the hidden launcher'}; $baseline=(Get-ScheduledTaskInfo -TaskName $name).LastRunTime; $until=(Get-Date).AddSeconds(80); do {Start-Sleep -Seconds 2; $info=Get-ScheduledTaskInfo -TaskName $name; if($info.LastRunTime -gt $baseline -and (Get-ScheduledTask -TaskName $name).State -ne 'Running'){if($info.LastTaskResult -ne 0){throw "Mount task failed: $($info.LastTaskResult)"}; Write-Output "Mount task ran successfully at $($info.LastRunTime)"; exit 0}} while((Get-Date) -lt $until); throw 'The mount task did not complete during the observation period'`;
    const result = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-EncodedCommand", Buffer.from(script, "utf16le").toString("base64")], {encoding: "utf8", windowsHide: true, timeout: 90_000});
    if (result.stdout) console.log(result.stdout);
    if (result.error || result.status !== 0) throw new Error(`Task observation failed (${result.status}): ${result.error?.message ?? result.stderr}`);
    return;
  }
  if (mode === "probe") {
    for (const name of names.slice(0, 3)) {
      const result = spawnSync("schtasks.exe", ["/Query", "/TN", name, "/FO", "LIST", "/V"], {encoding: "utf8", windowsHide: true, timeout: 20_000});
      if (result.error || result.status !== 0) throw new Error(`Task query failed for ${name}: ${result.error?.message ?? result.stderr}`);
      console.log(result.stdout);
    }
    return;
  }
  if (mode === "scheduled-test") {
    const name = `semio-quiet-test-${process.pid}`;
    const input = join(generated, `${name}.txt`);
    mkdirSync(generated, {recursive: true});
    let created = false;
    try {
      command("schtasks.exe", ["/Create", "/TN", name, "/SC", "ONCE", "/ST", "23:59", "/IT", "/TR", `wscript.exe "${join(import.meta.dirname, "diagnostic.vbs")}" ${name}`]);
      created = true;
      console.log(`Test action: ${/<Exec>[\s\S]*?<\/Exec>/u.exec(task(name))?.[0] ?? "missing"}`);
      command("schtasks.exe", ["/Run", "/TN", name]);
      const ps = `Start-Sleep -Seconds 5; $p=@(Get-CimInstance Win32_Process -Filter "Name='wscript.exe'" | Where-Object {$_.CommandLine -like '*${name}*'}); Write-Output "Matching script processes: $($p.Count)"; foreach($item in $p){$window=Get-Process -Id $item.ProcessId; Write-Output "Host window: $($window.MainWindowHandle) $($window.MainWindowTitle)"; $children=@(Get-CimInstance Win32_Process -Filter "ParentProcessId=$($item.ProcessId)"); foreach($child in $children){Write-Output "Child: $($child.Name)"}}`;
      const processes = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-EncodedCommand", Buffer.from(ps, "utf16le").toString("base64")], {encoding: "utf8", windowsHide: true, timeout: 30_000});
      if (processes.error || processes.status !== 0) throw new Error(`Test process query failed: ${processes.error?.message ?? processes.stderr}`);
      console.log(processes.stdout);
      const deadline = Date.now() + 40_000;
      let last = "unavailable";
      while (Date.now() < deadline) {
        const result = spawnSync("schtasks.exe", ["/Query", "/TN", name, "/FO", "LIST", "/V"], {encoding: "utf8", windowsHide: true, timeout: 10_000});
        if (result.error || result.status !== 0) throw new Error(`Test task query failed: ${result.error?.message ?? result.stderr}`);
        const code = /^Last Result:\s*(-?\d+)\s*$/mu.exec(result.stdout)?.[1];
        last = code ?? result.stdout;
        if (code === "17") {
          console.log("Task Scheduler received the hidden child's exit code 17");
          return;
        }
        Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 1000);
      }
      const actual = task(name);
      throw new Error(`The test task did not return the expected exit code: ${last}; action: ${/<Exec>[\s\S]*?<\/Exec>/u.exec(actual)?.[0] ?? "missing"}`);
    } finally {
      if (existsSync(`${input}.debug`)) console.log(readFileSync(`${input}.debug`, "utf8"));
      if (created) command("schtasks.exe", ["/Delete", "/TN", name, "/F"]);
      if (existsSync(input)) unlinkSync(input);
      if (existsSync(`${input}.debug`)) unlinkSync(`${input}.debug`);
    }
  }
  if (mode === "processes") {
    const script = `$ErrorActionPreference='Stop'; $all=@(Get-CimInstance Win32_Process); $roots=@($all | Where-Object {$_.Name -eq 'wscript.exe' -and $_.CommandLine -like '*quiet-tasks*'}); if($roots.Count -eq 0){throw 'No fleet launcher was running during observation'}; foreach($root in $roots){$kind=if($root.CommandLine -like '*replica.txt*'){'replica'}elseif($root.CommandLine -like '*mount.txt*'){'mount'}else{'other'}; Write-Output "Fleet launcher: $kind"}; $ids=@($roots.ProcessId); do {$old=$ids.Count; $ids=@($ids + @($all | Where-Object {$ids -contains $_.ParentProcessId} | ForEach-Object {$_.ProcessId}) | Select-Object -Unique)} while($ids.Count -gt $old); foreach($id in $ids){$p=Get-Process -Id $id -ErrorAction SilentlyContinue; if($p){Write-Output "$($p.ProcessName) $id window=$($p.MainWindowHandle)"; if($p.MainWindowHandle -ne 0){throw "Visible window for process $id"}}}`;
    const result = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-EncodedCommand", Buffer.from(script, "utf16le").toString("base64")], {encoding: "utf8", windowsHide: true, timeout: 60_000});
    if (result.stdout) console.log(result.stdout);
    if (result.error || result.status !== 0) throw new Error(`Process observation failed (${result.status}): ${result.error?.message ?? result.stderr}`);
    return;
  }
  if (mode === "clean-test") {
    const directory = import.meta.dirname.replaceAll("'", "''");
    const script = `$ErrorActionPreference='Stop'; $p=@(Get-CimInstance Win32_Process -Filter "Name='wscript.exe'" | Where-Object {$_.CommandLine -like '*${directory}*' -and $_.CommandLine -like '*semio-quiet-test-*'}); foreach($item in $p){Write-Output "Stopping test process $($item.ProcessId)"; Stop-Process -Id $item.ProcessId}`;
    const result = spawnSync("powershell.exe", ["-NoProfile", "-NonInteractive", "-EncodedCommand", Buffer.from(script, "utf16le").toString("base64")], {encoding: "utf8", windowsHide: true, timeout: 60_000});
    if (result.stdout) console.log(result.stdout);
    if (result.error || result.status !== 0) throw new Error(`Test process cleanup failed: ${result.error?.message ?? result.stderr}`);
    for (const file of ["semio-quiet-test-58120.txt", "semio-quiet-test-58120.txt.debug", "semio-quiet-test-46948.txt.debug", "test-command.txt"]) {
      const path = join(generated, file);
      if (existsSync(path)) unlinkSync(path);
    }
    return;
  }
  if (mode === "cleanup") {
    for (const name of names) if (existsSync(join(taskRoot, name)) && task(name).includes(helper)) throw new Error(`${name} still uses the hidden launcher`);
    for (const file of [helper, join(quietRoot, "semio-fleet-store-mount.txt"), join(generated, "test-command.txt"), join(generated, "semio-fleet-store-mount.previous.xml"), join(generated, "semio-fleet-store-mount.xml")]) if (existsSync(file)) unlinkSync(file);
    for (const directory of [quietRoot, generated]) if (existsSync(directory)) rmdirSync(directory);
    console.log("Removed files from the unsuccessful attempt");
    return;
  }
  if (mode === "test") {
    const sample = "<Task><Triggers>once</Triggers><Actions><Exec><Command>cmd.exe</Command><Arguments>/c echo a&amp;b</Arguments><WorkingDirectory>C:\\semio</WorkingDirectory></Exec></Actions></Task>";
    const converted = headless(sample, "sample");
    if (converted.line !== '"cmd.exe" /c echo a&b' || !converted.xml.includes("<Triggers>once</Triggers>") || headless(converted.xml, "sample").line !== undefined) throw new Error("Transformation test failed");
    mkdirSync(generated, {recursive: true});
    const input = join(generated, "test-command.txt");
    writeXml(input, 'cmd.exe /d /c exit 17');
    for (const [host, script, flags] of [["cscript.exe", join(import.meta.dirname, "quiet.vbs"), ["//nologo"]], ["wscript.exe", helper, []]] as const) {
      const result = spawnSync(host, [...flags, script, input], {windowsHide: true, timeout: 30_000});
      if (result.error || result.status !== 17) throw new Error(`${host} launcher test failed: ${result.error?.message ?? result.status}`);
    }
    console.log("Transformation and headless exit-code tests passed");
    return;
  }
  if (mode !== "inspect" && mode !== "apply") throw new Error("Use test, inspect, apply, request, or cleanup");
  const installed = names.filter(name => existsSync(join(taskRoot, name)));
  if (!installed.includes(names[0]!) || !installed.includes(names[1]!)) throw new Error("Fleet mount or replica task is missing");
  for (const name of installed) {
    const before = task(name);
    const after = headless(before, name);
    console.log(`${name}: ${after.line === undefined ? "headless" : "visible console"}`);
    if (mode !== "apply" || after.line === undefined) continue;
    mkdirSync(generated, {recursive: true});
    mkdirSync(quietRoot, {recursive: true});
    writeFileSync(helper, readFileSync(join(import.meta.dirname, "quiet.vbs")));
    writeXml(join(quietRoot, `${name}.txt`), after.line);
    const backup = join(generated, `${name}.previous.xml`);
    const update = join(generated, `${name}.xml`);
    writeXml(backup, before);
    writeXml(update, after.xml);
    if (task(name) !== before) throw new Error(`${name} changed while preparing it`);
    command("schtasks.exe", ["/Create", "/TN", name, "/XML", update, "/F"]);
    const actual = task(name);
    if (headless(actual, name).line !== undefined) {
      command("schtasks.exe", ["/Create", "/TN", name, "/XML", backup, "/F"]);
      throw new Error(`${name} did not retain the headless action; restored previous definition`);
    }
    console.log(`${name}: updated`);
  }
}

main();
