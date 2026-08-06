# Wave 3 enforcement — env gate

`policyOsStateAuthorityBreaches` and `policyDocumentAppShapeBreaches` are implemented in root `📜️script.ts` but registered in `export const policy` **only when** `SEMIO_OS_STATE_AUTHORITY=1`.

**Why gated:** Wave 2 is incomplete — probing with the flag on finds ~101–102 `os-state-authority/*` high-priority breaches. Unconditional registration would fail the default policy/verify path.

**Flip condition:** When both functions report **zero** breaches, remove the `process.env.SEMIO_OS_STATE_AUTHORITY === "1"` guard and apply snippets in `🧪w3-enforcement-draft.md`.

**Probe:**
```bash
bun ./.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/🧪w3-run-policies.ts .
SEMIO_OS_STATE_AUTHORITY=1 bun ./.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/🧪w3-run-policies.ts .
```
