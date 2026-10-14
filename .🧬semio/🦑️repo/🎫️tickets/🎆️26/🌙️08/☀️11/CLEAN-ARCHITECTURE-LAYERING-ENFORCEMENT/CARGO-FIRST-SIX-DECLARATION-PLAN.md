# First Six Neutral Cargo Execution Declarations

Read-only design derived from the existing general-native-cargo-census.md and bounded reads of these six owners. No imports, tests, issuer/factory calls, compiler runs, production edits, fixtures or migration scripts. These declarations are reviewable future authored data, not applied files.

## Scope and Current Selection Contract

Choose Trace, Async Macros, Value Derive, Schema Derive, Schema State and Canvas. Each script has one standard test command and a single Repo test import; no exact/build/lint/command branch. Exclude 2d from this first simple batch: it interprets `compute` and `compute keys` and prepends Cargo filters, requiring a distinct selection design rather than a blind tail.

Current build inference `🏗️build/🕸️graph/📥️inference/🟨️.mjs:1070–1074` records an absolute first Bun script token. The current General selector explicitly expands only an anchored `{workspaceRoot}/...` token using its required workspaceRoot parameter, and the Repo issuer supplies the original root. The Actor declaration already uses that contract. Each proposed declaration below therefore uses `{workspaceRoot}/<owner>/📜️script.ts`, matching the inferred physical command without an alias/fallback or machine-specific authored path. The earlier draft blocker observation was superseded after inspecting this concurrent selector correction; no normalization repair is required for this branch.

## Common Intended General Route

Replace the Repo import with General Process public facade `🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts` using its actual relative depth. Keep router/default command/receive original invocation. Call executeCargoRequestV1 with `{version:1,kind:"tests",manifest:<repo-relative manifest>,cwd:<repo-relative owner>,packages:[<exact package>],arguments:segments}` and the original invocation. Remove the General script policy-reader and resolveTestLevel call for this route. Retain raw segments: the endpoint owns resolveTestLevel, consumes a recognized leading fundamental/quick/long/exhaustive, preserves rest exactly, and inherits the existing SEMIO_TEST_LEVEL when no level token is present. Stripping level in General would disagree with the descriptor selected from full forwarded argv. No compiler/network/storage/environment authority enters request data.

Each package owns three required edits: update its 📜️script.ts; add cargo-execution = "🧪️execution/🔣️.json" inside its existing [package.metadata.semio] table (do not duplicate table); add its authored 🧪️execution/🔣️.json consuming canonical General declaration schema. Existing target options/forwarding should remain, with required project inputs including the authored declaration through projectRoot sources. No Source metadata should be added to these physical Cargo tests merely to bypass native admission. Existing graph inference supplies nativeOwnerCommand/backend injection using the explicit anchored workspace token.

## ⏱️trace

Owner: `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust`. Project target: `@semio-tech/framework-trace-rs:test`. Exact Cargo package: `semio-framework-trace`.

Current target: `{"cwd": "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test"}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-trace"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `c01ba534f3f5ab1cf0d22d6dc29af3d06a5120c3ec41cc9573fc818ea32e0b8d`.
- `Cargo.toml`: `0ab9053e508b322329b9350fcd005b4540aaa8b79e3f919b7810e87801f0c00f`.
- `📋️project.json`: `593058ea51b0f3886496a5d0e5340c487779d07900aa670dd3ea87428bcafa50`.

## ⏳️async/✨️macros

Owner: `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust`. Project target: `@semio-tech/async-macros-rs:test`. Exact Cargo package: `semio-framework-async-macros`.

Current target: `{"cwd": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-async-macros"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `d31976b2e52111618fca5c2aa565594bf0b48f2c4f13d40b04c390c2753dbfd9`.
- `Cargo.toml`: `36a96de0a09fdab16aaf0e806077ed2025ae799738d6ce17d49ff8a3f1663e49`.
- `📋️project.json`: `3f254f30d3d7900cee55c415565a1afab859c756e9cc97144e9d5de9c370ed99`.

## 🌱️value/✨️derive

Owner: `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust`. Project target: `@semio-tech/value-derive-rs:test`. Exact Cargo package: `semio-framework-value-derive`.

Current target: `{"cwd": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-value-derive"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `15e603584b37f4e69f8e82b62d623fde338f814b732ebe126ffd9b1d6b1b1da9`.
- `Cargo.toml`: `d9099b5f56f4aeb4dcc019c6ca9fd3cef6397aabfc3639bdcd8f7ee5660b176a`.
- `📋️project.json`: `b933f08da645e4f50191396082b4f061a0e0ae4c8df585f76db688631756b05c`.

## 🧬️schema/✨️derive

Owner: `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust`. Project target: `@semio-tech/schema-derive-rs:test`. Exact Cargo package: `semio-framework-schema-derive`.

Current target: `{"cwd": "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-schema-derive"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `82f9dab90636b8e7b495973b3e5f55d944ed3fca25c4a5dffad6e1fef6719e01`.
- `Cargo.toml`: `b720a371714924ffb42c02de82f2efd255523722d854cb82d74e502cf7b579a1`.
- `📋️project.json`: `a97c659512661592e6c27125b0176b29ec55fc1352a29075f7dfd522203ccceb`.

## 🧬️schema/📶️state

Owner: `🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust`. Project target: `@semio-tech/schema-state-rs:test`. Exact Cargo package: `semio-framework-schema-state`.

Current target: `{"cwd": "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-schema-state"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `01ec39763eb0cbafa04c50e04afa0b97efeb69df41b658297403e3f6e474d141`.
- `Cargo.toml`: `a4fa1f33b88e085ea25c9e21c6a6ebe7d7d7c4586264f4c79ca1aad8e313eab6`.
- `📋️project.json`: `36683ffc736c66774ce353ed30ae80fec8eaec6c578874636b0492043066cf5c`.

## 🖼️canvas

Owner: `🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust`. Project target: `semio-framework-canvas:test`. Exact Cargo package: `semio-framework-canvas`.

Current target: `{"cwd": "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test", "forwardAllArgs": true}`. No target defaults are executed by this audit. Current buildExecutionArgumentsV1 forwards arguments unless explicitly false; Trace omits forwardAllArgs and therefore follows that existing true behavior, while the other five explicitly set true.

Portable anchored owner selection and empty-tail request:

```json
{
  "version": 1,
  "operations": [
    {
      "command": {
        "program": "bun",
        "cwd": "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust",
        "arguments": [
          "{workspaceRoot}/🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/📜️script.ts",
          "test"
        ],
        "tail": "test-arguments"
      },
      "request": {
        "version": 1,
        "kind": "tests",
        "manifest": "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/Cargo.toml",
        "cwd": "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust",
        "packages": [
          "semio-framework-canvas"
        ],
        "arguments": []
      }
    }
  ]
}
```

With forwarded raw tail `A`, declaration selection produces the identical request with arguments `A`; caller request must use the same raw `segments`. Manifest and cwd remain exactly the paths above. The default router command is test, but the authored selected graph task includes the literal `test` argv and expanded anchored absolute script token; do not add an implicit-command or ./ alias.

Observed SHA-256:

- `📜️script.ts`: `b0f9e24c9bc95943ffe388217532c47552928b37b83fa838ebf5c5937b611ba3`.
- `Cargo.toml`: `799a2d812a973bf1a74252512bc34e2e0bc83974ceba5c9ac43528762d431a6f`.
- `📋️project.json`: `9ee0cf443941a3618643c2836a0de231ef6a5afea70cf7bc13882cad386ecdf7`.

## Validation Required Before Rollout

After the current Actor qualification implement one module at a time against the canonical closed declaration/request schema and existing language-agnostic contract vectors plus Ajv oracle. Verify declaration exact matching from the actual registered graph command, forwarded test-level/rest preservation, missing/mismatched backend refusal before effect, original signal/deadline/native grants, and absence of General-to-Repo imports. Native qualification must use a fresh original authored launch; this document claims none. No new Source capture or acceptance was started.

Selector snapshot SHA-256: `534c908b879df578e5184f6cdadb16fe1d1c3e8c89f643424a731a06f4f6c08f`. Current issuer passes root explicitly at Cargo execution line51.


## October15 Source Rollout Status

Trace and Async Macros are now implemented, source-only: [current two-consumer report](cargo-two-consumers-current.md). Common API unchanged; full endpoint suite63pass/187expectations, owning strict/noUnchecked398Sources0diagnostics. Current actual registered graph failures remain independent pre-assertion frontiers. Earlier quoted hashes/declarations in this plan are historical design observations, not the new frozen roster. Value Derive, Schema Derive, Schema State and Canvas remain future handcrafted consumers. No current native acceptance is claimed.


Allsix planned simpleconsumers now have actualSource implementation: [current report](cargo-six-consumers-current.md). Finalendpoint67pass/279expectations, strict/noUnchecked402Sources0diagnostics. Current21file roster named inreport supersedes prior designobservations/two-consumer currentness; historical receipts retained. No registered/native qualification is implied.
