#!/usr/bin/env python3
"""Generates staged/protocol.json (the closed sandbox protocol and scanner rules). usage: build_protocol.py"""
import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
WR_SYNC="writeFileSync|appendFileSync|mkdirSync|rmSync|rmdirSync|unlinkSync|renameSync|copyFileSync|cpSync|truncateSync|symlinkSync|linkSync|createWriteStream"
WR_ASYNC="writeFile|appendFile|mkdir|rm|rmdir|unlink|rename|copyFile|cp|truncate|symlink|link"
FIRST="writeFileSync|appendFileSync|mkdirSync|rmSync|rmdirSync|unlinkSync|renameSync|truncateSync|createWriteStream|writeFile|appendFile|mkdir|rm|rmdir|unlink|rename|truncate|Bun\\.write"
SECOND="cpSync|copyFileSync|cp|copyFile|symlinkSync|symlink|linkSync|link|renameSync|rename"
WR="\\b("+WR_SYNC+"|"+WR_ASYNC+"|Bun\\.write)\\("
def first(anchor): return "\\b("+FIRST+")\\(\\s*((path\\.)?(join|resolve)\\(\\s*)?"+anchor
def second(anchor): return "\\b("+SECOND+")\\(\\s*[^,;\\n]{1,120},\\s*((path\\.)?(join|resolve)\\(\\s*)?"+anchor
WIN="[^;\\n]{0,200}?"
ANCHOR_DIRECT="(process\\.cwd\\(\\)|import\\.meta\\.(dir|dirname|path|url)\\b|__dirname|__filename)"
ANCHOR_NAMED="\\b(repoRoot|repositoryRoot|REPO_ROOT|workspaceRoot|WORKSPACE_ROOT|checkoutRoot|SEMIO_WORKSPACE_ROOT)\\b"
CACHE="(^|[^A-Za-z0-9_])\\.cache(/|\\b)|⚡️cache|\\.🧬semio/🦑️repo/⚡️cache"
SANDBOX="\\b(mkdtemp(Sync)?|tmpdir\\(\\)|createTestSandbox\\w*|withTestSandbox\\w*|SEMIO_TEST_SANDBOX|SEMIO_TEST_ARTIFACT_DIR|testSandbox\\w*|repoTestArtifactEnvironment|temporaryDirectory|withTemporary\\w*|createTemporary\\w*|scratchRoot\\w*|ScratchRoot\\w*)\\b"
GO_SANDBOX="\\b(t|b)\\.TempDir\\(\\)|os\\.MkdirTemp|ioutil\\.TempDir|os\\.CreateTemp|sandbox\\.|withMonorepoFixture|withExhaustiveRepo|withExhaustiveMonorepo|withTempRepo|scratchRepo|withGoalFixture|withSealedFixture|withSectionFixture|initTestGitRepo|setupTicketDir|setupImportantTicket|testStore|ctx\\.(WorkDir|ArtifactDir)|WorkDir"
RS_SANDBOX="\\b(temp_dir|tempfile|TempDir|CARGO_TARGET_TMPDIR|scratch|unique_dir|SEMIO_TEST_SANDBOX|SEMIO_TEST_ARTIFACT_DIR)\\b"
LANGS=["ts","go","rs","py","sh"]
def msg(en,de): return {"en":en,"de":de}
rules=[
 {"id":"checkout-write","severity":"error","languages":["ts"],"scope":"line","pattern":first(ANCHOR_DIRECT)+"|"+second(ANCHOR_DIRECT),
  "message":msg("Writes relative to the process directory or the test source directory, i.e. into the real checkout.","Schreibt relativ zum Prozessverzeichnis oder Testquellverzeichnis, also in den echten Checkout.")},
 {"id":"checkout-write-named","severity":"warn","languages":["ts"],"scope":"line","pattern":first(ANCHOR_NAMED)+"|"+second(ANCHOR_NAMED),
  "message":msg("Writes below a variable named like the repository root; it must be a sandbox clone.","Schreibt unterhalb einer Variable, die wie die Repository-Wurzel heisst; sie muss ein Sandbox-Klon sein.")},
 {"id":"cache-write","severity":"error","languages":["ts"],"scope":"line","pattern":first("("+CACHE+")")+"|"+second("("+CACHE+")"),
  "message":msg("Mutates the shared repository cache or the native cache partitions.","Veraendert den geteilten Repository-Cache oder die nativen Cache-Partitionen.")},
 {"id":"unsandboxed-writer","severity":"warn","languages":["ts"],"scope":"file","all":["\\b(writeFileSync|appendFileSync|mkdirSync|rmSync|renameSync|copyFileSync|cpSync|Bun\\.write)\\("],"none":[SANDBOX],
  "message":msg("Mutates the file system but never obtains a temporary or sandbox root.","Veraendert das Dateisystem, bezieht aber nie eine temporaere oder Sandbox-Wurzel.")},
 {"id":"real-cache-path","severity":"warn","languages":LANGS,"scope":"line","pattern":"(^|[^A-Za-z0-9_])\\.cache/b(\\b|/)|⚡️cache|\\.🧬semio/🦑️repo/⚡️cache",
  "message":msg("Names the shared native cache partitions or the repository cache directly.","Benennt die geteilten nativen Cache-Partitionen oder das Repository-Cache-Verzeichnis direkt.")},
 {"id":"ticket-scratch-root","severity":"error","languages":["ts"],"scope":"line","pattern":"(process\\.cwd\\(\\)|import\\.meta\\.(dir|dirname)|repoRoot|workspaceRoot)[^;\\n]{0,80}🎫️tickets[^;\\n]{0,160}🗑️generated",
  "message":msg("Uses the generated folder of a real ticket as scratch root; the folder is shared and swept by others.","Nutzt den generated-Ordner eines echten Tickets als Scratch-Wurzel; der Ordner ist geteilt und wird von anderen aufgeraeumt.")},
 {"id":"ticket-state-path","severity":"warn","languages":LANGS,"scope":"line","pattern":"🎫️tickets|🥅️goals",
  "message":msg("Names the real ticket/goal state; tests must use a throwaway repository.","Benennt den echten Ticket-/Ziel-Zustand; Tests muessen ein Wegwerf-Repository nutzen.")},
 {"id":"persistent-native-state","severity":"error","languages":["ts"],"scope":"line",
  "pattern":"(^|[^\"'`A-Za-z0-9_$])(withNativePersistentStateV1|withNativePersistentStateTaskV1|openPersistentNativeImageV1|withPersistentNative\\w*StorageV1|withRepositoryDevelopmentState\\w*|withDevLocalHubSessionV1)\\s*[(<]",
  "message":msg("Opens a persistent partition or state child that outlives the process.","Oeffnet eine persistente Partition oder ein Zustandskind, das den Prozess ueberlebt.")},
 {"id":"native-image-unsandboxed","severity":"error","languages":["ts"],"scope":"file","all":["(^|[^\"'`A-Za-z0-9_$])(withNativeImageStorageV1|openNativeImageStorageV1|publishNativeImagePublicationV1|openNativeDirectoryImageStorageV1|withIssuedNativeImageV1)\\s*[(<]|\\b(spawnSync|execFileSync|spawn|execFile|runCmd|captureOwnedCommandV1)\\(\\s*[\"']hdiutil[\"']"],"none":[SANDBOX],
  "message":msg("Creates or attaches disk images without any sandbox root, so a crash leaks a mount.","Erzeugt oder haengt Abbilder ein ohne Sandbox-Wurzel, ein Absturz hinterlaesst einen Mount.")},
 {"id":"fixed-port-bind","severity":"error","languages":LANGS,"scope":"line",
  "pattern":"\\b(listen|serve|Listen|ListenAndServe|ListenTCP)\\(\\s*(\\{[^}\\n]{0,120}\\bport\\s*:\\s*)?[1-9][0-9]{3,4}\\b|\\b(TcpListener|UdpSocket|UnixListener)::bind\\(\\s*\"[0-9.:a-zA-Z\\[\\]]*:[1-9][0-9]{3,4}\"|\\bnet\\.Listen\\(\"tcp[46]?\",\\s*\"[0-9.]*:[1-9][0-9]{3,4}\"|--port[ =]+[1-9][0-9]{3,4}\\b|\\bPORT=[1-9][0-9]{3,4}\\b",
  "message":msg("Binds a fixed TCP port; two concurrent runs collide.","Bindet einen festen TCP-Port; zwei gleichzeitige Laeufe kollidieren.")},
 {"id":"fixed-port-config","severity":"warn","languages":["ts","py","sh"],"scope":"line","pattern":"\\bport\\s*(:|=)\\s*[1-9][0-9]{3,4}\\b","flags":"i","also":["(serve|listen|server|host|vite|http|websocket|socket)"],"exempt":["(mock|stub|refus|reject)"],
  "message":msg("Configures a fixed TCP port for a server.","Konfiguriert einen festen TCP-Port fuer einen Server.")},
 {"id":"fixed-port-url","severity":"warn","languages":LANGS,"scope":"line","pattern":"(localhost|127\\.0\\.0\\.1|\\[::1\\]|0\\.0\\.0\\.0):[1-9][0-9]{3,4}\\b",
  "message":msg("Names a fixed local TCP port in an address.","Nennt einen festen lokalen TCP-Port in einer Adresse.")},
 {"id":"fixed-tmp-dir","severity":"error","languages":LANGS,"scope":"line",
  "pattern":"\\b(join|resolve)\\(\\s*(os\\.)?tmpdir\\(\\)\\s*,\\s*[\"'`][^\"'`$\\n]+[\"'`]\\s*\\)|temp_dir\\(\\)\\s*\\.join\\(\"[^\"{]*\"\\)|filepath\\.Join\\(os\\.TempDir\\(\\),\\s*\"[^\"]+\"\\)|[\"'`](/tmp|/private/tmp|/var/tmp)/[A-Za-z0-9._-]+[\"'`]\\s*[,)]\\s*(\\{[^}\\n]*recursive|[^;\\n]{0,40}(mkdir|writeFile|rmSync|rm\\())",
  "exempt":["mkdtemp","MkdirTemp","tempfile","\\bexpect\\("],
  "message":msg("Uses a constant name below the shared temp directory; two concurrent runs collide.","Nutzt einen konstanten Namen im gemeinsamen Temp-Verzeichnis; zwei gleichzeitige Laeufe kollidieren.")},
 {"id":"fixed-tmp-literal","severity":"warn","languages":LANGS,"scope":"line","pattern":"[\"'`](/tmp|/private/tmp|/var/tmp)/[A-Za-z0-9._{$-]",
  "exempt":["mkdtemp","MkdirTemp","tempfile"],
  "message":msg("Names a literal path below the shared temp directory.","Benennt einen literalen Pfad unterhalb des gemeinsamen Temp-Verzeichnisses.")},
 {"id":"port-variable-bind","severity":"warn","languages":["ts"],"scope":"line","pattern":"\\.listen\\(\\s*(port|Number\\([^)]*\\.port\\))\\s*,|\\bBun\\.serve\\(\\{[^}\\n]{0,80}\\bport\\s*[,}]",
  "message":msg("Rebinds a port that was probed earlier; reserve it through the sandbox port lease instead.","Bindet einen zuvor sondierten Port erneut; ueber die Sandbox-Portreservierung reservieren.")},
 {"id":"real-home","severity":"warn","languages":LANGS,"scope":"line","pattern":"\\bhomedir\\(\\)|process\\.env\\.(HOME|USERPROFILE)\\b|os\\.UserHomeDir|dirs::home_dir|env::var\\(\"HOME\"\\)",
  "message":msg("Reads or writes below the real home directory.","Liest oder schreibt unterhalb des echten Home-Verzeichnisses.")},
 {"id":"global-process-kill","severity":"error","languages":LANGS,"scope":"line","pattern":"\\b(pkill|killall)\\b","flags":"i","exempt":["(expect|assert|toBe|toEqual|toContain|refus|forbid|never)"],
  "message":msg("Kills processes by name; it would hit other agents' processes.","Beendet Prozesse nach Name; trifft Prozesse anderer Agenten.")},
 {"id":"process-global-state","severity":"warn","languages":["ts","go"],"scope":"line","pattern":"\\bprocess\\.chdir\\(|\\bos\\.Chdir\\(",
  "message":msg("Changes the process working directory; parallel tests in one process interfere.","Aendert das Arbeitsverzeichnis des Prozesses; parallele Tests im selben Prozess stoeren sich.")},
 {"id":"go-unsandboxed-writer","severity":"error","languages":["go"],"scope":"file","all":["\\bos\\.(WriteFile|MkdirAll|Mkdir|Create|Remove|RemoveAll|Rename|OpenFile)\\("],"none":[GO_SANDBOX],
  "message":msg("Go test mutates the file system without t.TempDir(), a sandbox root or a fixture repository.","Go-Test veraendert das Dateisystem ohne t.TempDir(), Sandbox-Wurzel oder Fixture-Repository.")},
 {"id":"go-checkout-relative","severity":"error","languages":["go"],"scope":"line","pattern":"\\bos\\.(WriteFile|MkdirAll|Mkdir|Create|Remove|RemoveAll|Rename|OpenFile)\\([^;\\n]{0,200}?(\\.\\./|os\\.Getwd|runtime\\.Caller)","exempt":["tmpDir","TempDir","tempDir","WorkDir"],
  "message":msg("Go test writes relative to the source tree.","Go-Test schreibt relativ zum Quellbaum.")},
 {"id":"go-owner-unsandboxed","severity":"error","languages":["go"],"scope":"owner","all":["func Test\\w+\\(t \\*testing\\.T\\)","\\b(os\\.Getwd|workspace\\.(FindRepoRoot|GetRootDir|ResolveRepoRoot)|runtime\\.Caller)\\b"],"none":["\\bsandbox\\.Run\\("],
  "message":msg("The Go test owner has no TestMain bound to sandbox.Run, so its tests may resolve the real checkout.","Der Go-Testbesitzer hat kein an sandbox.Run gebundenes TestMain, seine Tests koennen den echten Checkout aufloesen.")},
 {"id":"rust-unsandboxed-writer","severity":"warn","languages":["rs"],"scope":"file","all":["\\b(fs::(write|create_dir_all|create_dir|remove_dir_all|remove_file|copy|rename)|File::create|OpenOptions::new)\\("],"none":[RS_SANDBOX],
  "message":msg("Rust test mutates the file system without a temporary or sandbox root.","Rust-Test veraendert das Dateisystem ohne temporaere oder Sandbox-Wurzel.")},
 {"id":"rust-checkout-write","severity":"error","languages":["rs"],"scope":"line","pattern":"\\b(fs::(write|create_dir_all|create_dir|remove_dir_all|remove_file|copy|rename)|File::create)\\([^;\\n]{0,200}?(CARGO_MANIFEST_DIR|current_dir\\(\\))",
  "message":msg("Rust test writes relative to the crate or working directory.","Rust-Test schreibt relativ zum Crate- oder Arbeitsverzeichnis.")},
]
WRITE_TARGETS={"writeFileSync":[0],"appendFileSync":[0],"mkdirSync":[0],"rmSync":[0],"rmdirSync":[0],"unlinkSync":[0],"renameSync":[0,1],"copyFileSync":[1],"cpSync":[1],"truncateSync":[0],"symlinkSync":[1],"linkSync":[1],"createWriteStream":[0],"writeFile":[0],"appendFile":[0],"mkdir":[0],"rm":[0],"rmdir":[0],"unlink":[0],"rename":[0,1],"copyFile":[1],"cp":[1],"truncate":[0],"symlink":[1],"link":[1]}
protocol={
 "version":1,"owner":"repository:test-sandbox",
 "environment":{"sandbox":"SEMIO_TEST_SANDBOX","artifacts":"SEMIO_TEST_ARTIFACT_DIR","images":"SEMIO_TEST_IMAGE_ROOT","cache":"SEMIO_TEST_CACHE_ROOT","guard":"SEMIO_TEST_GUARD","report":"SEMIO_TEST_GUARD_REPORT","checkout":"SEMIO_TEST_CHECKOUT_ROOT"},
 "layout":{"parent":"st","owner":"owner.json","directories":["artifacts","tmp","images","cache"],"maximumRootLength":90,"heartbeatMilliseconds":30000,"staleHeartbeatMilliseconds":3600000,"graceMilliseconds":60000,"reclaimIntervalMilliseconds":300000,"stamp":"reclaim.stamp"},
 "ports":{"minimum":49152,"maximum":65000,"attempts":64,"directory":"ports"},
 "guard":{
  "modes":["off","report","enforce"],"defaultMode":"off",
  "writeTargets":WRITE_TARGETS,
  "openFunctions":["openSync","open"],
  "kernelPrograms":["hdiutil","diskutil"],
  "killPrograms":["pkill","killall"],
  "kernelVerbs":["attach","create","convert","resize","mount","detach","eject","unmount"],
  "protectedCheckoutPrefixes":[".cache",".🧬semio",".git","node_modules"],
  "allowedCheckoutPrefixes":[".cache/b/o/m/tests"],
  "scratchRoots":["/private/tmp","/private/var/folders","/var/tmp","/tmp"]},
 "scan":{"testSegment":"🧪️tests","skipSegments":["🧫️fixtures","node_modules","dist","target","🗑️generated"],
  "extensions":{"ts":["ts","tsx","mts","cts","js","mjs","cjs"],"go":["go"],"rs":["rs"],"py":["py"],"sh":["sh","bash","zsh"]},
  "maximumBytes":1500000,
  "suite":{"ts":"from\\s*[\"'](bun:test|vitest|node:test)[\"']|registerBuildTestLawV1|\\b(test|it|describe)\\(","go":"func Test\\w+\\(|func Main\\(|package adapter","rs":"#\\[(test|tokio::test)\\]|#\\[cfg\\(test\\)\\]|fn main\\(","py":"\\bdef test_|import unittest|import pytest","sh":".*"},
  "commentLine":{"ts":"^\\s*(//|/\\*|\\*)","go":"^\\s*(//|/\\*|\\*)","rs":"^\\s*(//|/\\*|\\*)","py":"^\\s*#","sh":"^\\s*#"}},
 "rules":rules}
json.dump(protocol,open(HERE+"/staged/sb-protocol.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
open(HERE+"/staged/sb-protocol.json","a").write("\n")
print(len(rules),"rules")
