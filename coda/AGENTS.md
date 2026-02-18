# coda

coda is an ai that helps in designing buildings with Automated-Compliance-Checking (ACC).

# Specs

## project

A `project` is folder that contains a `design` which is changed by `agents` on which ACC is applied for certain `targets`.

A `project` MUST have exactly one `design` and MUST have at least one `target`.

A `project` MUST have exactly this structure:

```
├── .coda
│   ├── mcp-servers
│   │   └── {{MCPSERVERID}}{{(.exe)?}}
│   ├── validators
│   │   └── {{VALIDATORID}}{{(.exe)?}}
│   ├── runs
│   │   └── {{RUNTIMESTAMP}} e.g. `2026-02-10_11-47-02`
│   │       ├── run.json
│   │       └── iterations
│   │           └── {{ITERATIONINDEX}} e.g. `001`
│   │               ├── iteration.json
│   │               ├── targets
│   │               │   └── {{TARGETID}} e.g. `berlin-building-code-v1`
│   │               │       ├── trials
│   │               │       │   └── {{TRIAL}} e.g. `01`
├── .github
│   └── agents
│       └── coda-main-agent.agent.md
│       └── {{DESIGNID}}-to-{{TARGETID}}-translator-agent.agent.md
├── PROJECTFILESANDFOLDERS...
```

## format

`format` is a representation of a building design in a specific data format.

### design

`design` is a `format` used for authoring a building design. It MUST be openable by a `design software`.

### target

A `target` is a `format` used for validating a design.

A `target` MUST have exactly this structure:
It MUST inputable to a `validator`.

## system

A `system` is a computer executable action that can be used to perform a task.

### program

An `program` is deterministic system that can be used to perform a task.

#### design software

A `design software` is a program that can open a `design` and can be used to author a `design`.

Every `design software` MUST have an mcp server that can be used by `agents` to interact with the `design`.

#### validator

A `validator` is a program that can validate a `target`. It MUST accpet the `target` as input and produce a `report` as output.

### agent

An `agent` is a llm-based system that can be used to perform a task. E.g. a `.github/agents/*.agent.md` file.

#### coda

`coda` is the main agent that coordinates the main loop of all systems.

`coda` is responsible for calling all `subagents`. The output of a `subagent`

#### subagent

A `subagent` is an agent that is responsible for a specific task.

A `subagent` MUST write its output into exactly one file.

A `subagent` MUST use only other mcp servers to gather information.

A `subagent` MUST NOT read or modify files or folders of the `project`.

##### translator

A `translator` is a subagent that translates a `design` into a `target`.

##### fixer

A `fixer` is a subagent that tries to fix a `design` to be compliant with a `target` for a given `report`.

## task

A `task` is a unit of work that is performed by a `system`.

### run

A `run` is one execution loop of `coda`.

A `run` MUST BE inside one chat with `coda`.

### iteration

An `iteration` is one iteration of a `run`.

An `iteration` contains one `translation` and one `validation`.

### translation

A `translation` is a task perforemed by a `translator` that translates a `design` into a `target`.

### validation

A `validation` is a task perforemed by a `validator` that validates a `target` against a `design`.

There is an assistant go binary that calls `translators` (each translator is an agent) and `validators` (each validator is a binary) to check if a design is compliant. The result from a `translator` is directly piped into the `validator`. Every `translator` and `validator` pair is concurrently called. The assistant fans out to all `translators` and as soon as a `translator` returns the assistant calls the `validator` with the result. Then it waits until all `validators` have returned for all `targets`. It aggregates the result to a `report`. If the `report`contains `breachs`, then the `report` is provided to the `changer` (agent) which changes the `design` over the design mcp server. The `changer` iterates as much as it can to fix all the breachs from the `report`. It uses both anylze tools and change tools from the design mcp. Once it thinks it fixed all the breachs, it signals the `assistant`. The `assistant` then calls the `translators` and `validators` again on the changed `design`.

In general there are `rules` which are validated by the `validators`. Every `rule` consists of `clauses`. A breach appears when one `clause` is not satisfied. There are `measures`

It should all be within one file `go/assistant/main.go`.

As example, the design format/authoring platform/mcp server is `semio` and the targets are `BerlinBuildingCode` and `RoomProgram`.
There is one translator
for `semio->BerlinBuildingCode`
and a validator
for `BerlinBuildingCode`.
There is one translator
for `semio->RoomProgram`
and a validator
for `RoomProgram`.

Make a detailed architectural plan that I can download.

new design assistant

.coda
runs
RUNTIMESTAMP
run.json
interactions
ITERATION
interaction.json
targets
TARGET
trials
TRIAL
trial.TARGETEXTENSION
error.json
target.TARGETEXTENSION
report.REPORTEXTENSION
report.json
design.DESIGNEXTENSION
fixed.DESIGNEXTENSION

# MCP Server

## Resources

### Measures

uri: `coda://measures`

List all measures that are available.

### Measure

uri: `coda://measure/{id}`

measure: "coda://measure/{id}"
targets: "coda://targets"
target: "coda://target/{id}"
properties: "coda://{target-id}/properties"
property:"coda://{target-id}/property/{id}"
rules: "coda://{target-id}/rules"
rule: "coda://{target-id}/rule/{id}"

project: "coda://project"
current-run: "coda://current-run"
current-report: "coda://report"
iterations: "coda://iterations"
current-iteration: "coda://current-iteration"

## Tools

start-run: "coda://start-run"
start-iteration: "coda://start-iteration"
translate: "coda://translate"
fix: "coda://fix"

## Prompts

change <prompt>

# Agents

semio-to-blnbo-translation-agent # Responsible for translating semio format to blnbo
semio-to-roomprogram-translation-agent # Responsible for translating semio format to roomprogram
semio-change-agent # Responsible for changing the semio with semio-mcp to fix breachs from the report
