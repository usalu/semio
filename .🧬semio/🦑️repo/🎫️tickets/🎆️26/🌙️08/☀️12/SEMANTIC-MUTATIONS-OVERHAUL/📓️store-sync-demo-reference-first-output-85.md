# First Reference Terminal Tool Output

This is the complete terminal output returned for exec session 49718, exit code 1. Tool wait duration is not the command duration. Original retained controller output and receipt remain authoritative for its internal stages.

```text
[DEBUG] Capturing reviewed verifier inputs; Rust and provenance execution are disabled.
[DEBUG] Evaluating 25 authored source-token cases with the captured D lexer.
[DEBUG] FAIL 21/21 /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/🧫️run-9jca2c/🔣️receipt.json
55 |     }
56 |     const projects = getProjects(projectGraph, nxArgs);
57 |     const projectsToRun = (0, get_command_projects_1.getCommandProjects)(projectGraph, projects, nxArgs);
58 |     projectsToRun.forEach((projectName) => {
59 |         const command = argv.reduce((cmd, arg) => cmd + `"${arg}" `, '').trim();
60 |         (0, child_process_1.execSync)(command, {
                                 ^
error: Command failed: "bun" ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-sync-demo-source-structure-75/📜️script.ts" "reference"
 signal: null,
 status: 1,
 output: [ null, null, null ],
    pid: 66884,
 stdout: null,
 stderr: null,

      at genericNodeError (node:child_process:998:13)
      at checkExecSyncError (node:child_process:458:27)
      at execSync (node:child_process:278:31)
      at <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:29)
      at forEach (1:11)
      at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
      at async <anonymous> (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:75)

```
