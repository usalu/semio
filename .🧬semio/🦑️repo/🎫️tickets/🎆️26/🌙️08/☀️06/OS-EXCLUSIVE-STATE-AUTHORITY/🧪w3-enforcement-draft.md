# Wave 3 Enforcement Draft (do not apply until zero OS breaches)

Prepared by Wave 3 integrator. **Not applied** to root configs — applying now would fail CI while Wave 2 migrations remain open.

Related: `policyOsStateAuthorityBreaches` / `policyDocumentAppShapeBreaches` in `📜️script.ts` (active only when `SEMIO_OS_STATE_AUTHORITY=1`).

Probe (2026-08-06): flag on → **101–102** `os-state-authority/*` breaches (`item-scope-global` ~23, `id-minting` ~32, `authority-struct-map` ~38, `document-app-shape` ~9). Flag off → **0** OS breaches from these rules.

---

## 1. `📜️script.ts` — VerifyScript gate (apply after zero-breach flip)

Insert inside `VerifyScript.runGate()` after existing catalog checks:

```ts
    console.log("[verify] OS exclusive state authority policies…");
    // Requires unconditional registration of policyOsStateAuthorityBreaches /
    // policyDocumentAppShapeBreaches in export const policy (remove SEMIO_OS_STATE_AUTHORITY gate first).
    runCmd("bun", ["./📜️script.ts", "policy"], { cwd: this.root, ...orchestratorBudgetOpts() });
```

Also remove the env gate around the two `breaches.push(...policyOs…)` calls so the default policy run enforces them.

---

## 2. `.dependency-cruiser.cjs` — `no-state-outside-os` (draft)

Append to `forbidden` (severity until TS state sites are migrated — would warn/error on many framework UI + CAD modules today):

```js
    {
      name: "no-state-outside-os",
      severity: "error",
      comment:
        "OS-exclusive state authority: ✏️s and non-OS 🧰️framework must not import OS host DocumentStore/session internals via deep relative paths — go through @semio-tech packages / public host APIs",
      from: {
        path: "^(✏️s/|🧰️framework/)",
        pathNot: "^🧰️framework/🛍️products/💻️os/",
      },
      to: {
        path: "^🧰️framework/🛍️products/💻️os/.*/(🏪️store|🖥️host)/",
        dependencyTypes: ["local"],
      },
    },
```

Note: dep-cruiser cannot see Rust item-scope globals; those stay in `policyOsStateAuthorityBreaches`.

---

## 3. `eslint.config.mjs` — TS mutable module state (draft)

Append after `...crossPackageRelativeOverrides()`:

```js
  {
    files: ["✏️s/**/*.{ts,tsx}", "🧰️framework/**/*.{ts,tsx}"],
    ignores: [
      "**/node_modules/**",
      "**/dist/**",
      "**/pkg/**",
      "🧰️framework/🛍️products/💻️os/**",
      "compose/**",
      "♻️mit-bestand/**",
      "🌎️hub/**",
    ],
    rules: {
      "no-restricted-globals": [
        "error",
        { name: "localStorage", message: "OS-exclusive state authority: persist via OS DocumentStore / host APIs, not localStorage." },
        { name: "sessionStorage", message: "OS-exclusive state authority: persist via OS DocumentStore / host APIs, not sessionStorage." },
        { name: "indexedDB", message: "OS-exclusive state authority: persist via OS DocumentStore / host APIs, not indexedDB." },
      ],
      "no-restricted-syntax": [
        "error",
        {
          selector: "Program > VariableDeclaration[kind='let'] > VariableDeclarator > Identifier",
          message: "OS-exclusive state authority: no module-level `let` mutable bindings outside the OS product.",
        },
        {
          selector: "Program > VariableDeclaration[kind='var']",
          message: "OS-exclusive state authority: no module-level `var` outside the OS product.",
        },
        {
          selector: "Program > VariableDeclaration > VariableDeclarator[init.type='NewExpression'][init.callee.name=/^(Map|Set|WeakMap|WeakSet)$/]",
          message: "OS-exclusive state authority: no module-scope new Map()/Set() outside the OS product.",
        },
        {
          selector: "ClassDeclaration[id.name=/Store$/]",
          message: "OS-exclusive state authority: *Store classes must live under the OS product or be deleted.",
        },
      ],
    },
  },
```

Tune selectors after a dry-run — module-level `let` may need allowlisting for genuine constants that ESLint cannot see as `const`.

---

## 4. `.vscode/launch.json` — entries (draft)

Follow existing `node-terminal` + emoji naming. Suggested group `2_verify` (or next free order in that family):

```json
    {
      "name": "⚖️policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./📜️script.ts policy",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "2_verify", "order": 10 }
    },
    {
      "name": "⚖️policy🎫️os-state-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "SEMIO_OS_STATE_AUTHORITY=1 bun ./📜️script.ts policy",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "2_verify", "order": 11 }
    },
    {
      "name": "✅verify🎛gate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./📜️script.ts verify gate",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "2_verify", "order": 20 }
    }
```

After the zero-breach flip, drop `⚖️policy🎫️os-state-authority` (or keep as alias) and make `⚖️policy` enforce OS rules unconditionally.

---

## 5. Flip checklist

1. Wave 2 agents clear inventory in `🧪inventory-core.md`.
2. `SEMIO_OS_STATE_AUTHORITY=1 bun $TICKET/🧪w3-run-policies.ts $TICKET` → `osStateAuthority: 0`.
3. Remove env gate in `export const policy`.
4. Apply sections 1–4 above.
5. `bun ./📜️script.ts verify gate` green.
