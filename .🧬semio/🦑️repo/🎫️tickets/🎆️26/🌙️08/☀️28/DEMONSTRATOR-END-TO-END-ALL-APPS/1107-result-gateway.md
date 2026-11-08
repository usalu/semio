STATUS: WAITING. W-gateway-101 (rc 1, log 🗑️generated/1105b/W-gateway-101.txt) after lock-113: the only error is upstream semio-framework-pack-json 🎛️controlled-and-borrowed-bridge/🦀️.rs:237 `error.kind()` (ValueError.kind is a field; E0599); claimed by packdebt (pack-json production). Everything downstream skipped, so the three crates have no verdict yet. Waiting for packdebt FIXED, then rerun as W-gateway-102.
W-gateway-102 (rc 1): pack-json now fine; only semio-os-store-engine-core (12 errors, unclaimed) failed. EDITED (claimed, gateway): store-components lifted/store: retire_owned_mutation and take_returned_snapshot_read_retirement keep their grant signature but now run the effect-shaped value API under an AuthoringWallet (cold path; callers inside turns do not charge the turn, to be migrated to effect later); batch fold prepare_prefix/reserve_capacity_one via wallet; ArtifactStoreBatchPublication gained receiving_fault* fields. Rerun W-gateway-103 (UNVERIFIED).
engine-bundle (16) EDITED-unverified by gateway: STEP_WORK/authored in 🧳️bundle root, with_effect on receiving at message/root sites, wallet in metadata; rerun W-gateway-104

## FINAL (wrap-up on coordinator request)
STATUS: NOT GREEN, UNVERIFIED. GATEWAY GREEN NOT posted.
- After lock-113: W-gateway-101 red only at pack-json bridge:237 (packdebt fixed it). W-gateway-102: only engine-core red (12); fixed by gateway, W-gateway-103 no longer fails there (VERIFIED by absence). W-gateway-103: engine-bundle (16) and engine-close (7) red; engine-close belongs to store7 (claimed 23:55).
- engine-bundle EDITED, compile result UNVERIFIED: W-gateway-104 (detached, pid not recorded) was still running with 0 errors so far. Check 🗑️generated/1105b/W-gateway-104.{rc,txt}.
- Caveat: engine-core retire_owned_mutation / take_returned_snapshot_read_retirement keep their grant signatures and run the effect API under an AuthoringWallet (turns are not charged); migrate callers to effect-shaped later.
- Not yet reached: the testing crates' own errors (store4 doubles, panede law, panefaults, applaws); engine-close must also compile first.
- Rerun: C/1105b-tools/chk.sh W-gateway-105 Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws
W-gateway-104 rc 1: engine-bundle 2 errors at 🦀️.rs:148 (error-mapping); fixed by gateway (UNVERIFIED), after that engine-bundle compiled otherwise. Rerun W-gateway-105.

## Step 1 (migration, EDITED-unverified)
engine-core retire_owned_mutation(owner,effect) and take_returned_snapshot_read_retirement(effect) are effect-shaped, no AuthoringWallet. Propagated: engine delegating-accessors, SpaceMember trait + artifact-store impl + no-members + space-members-macro + testing doubles. Next: callers (tool-run, plugin, tests) - each sits in a grant-shaped retired(cx,d,|grant|..) closure that already uses stale APIs.

## Remaining callers of the two now-effect-shaped APIs (all UNMIGRATED, each already red on other stale APIs)
- tool-run: 📒️driver-and-ledger:527, 👥️member-owners-and-visitors:166,309,331,373, 🎞️tick/🎮️operations:118, ♻️retirement/📒️entry:60. They live in `retired(cx,d,|grant|..)` closures and use self.grant plus grant-shaped close_factory_ticket/admit_original(..).result; needs a crate-wide port to `cx.with_original_effect`/StepContext effects (owner: toolrun/toolsplit lane), not a call-site swap.
- plugin: 🧵️instance/📄️docs/📥️staging/🚪️ingress:41, 🪟️window/🎚️config/🗄️store-owners:69 (typedpack), 🪆️child/♻️child-retirement-and-return-pump:89.
- tests: interaction 📃️query/📡️live/📖️capture tests (use recipient.remaining_grant()), window-config 🪟️retained-window-config, store 🧪️tests/🌆️bin-02/🔬️unit:514.
VERIFY (not run): C/1105b-tools/chk.sh W-gateway-105 Cargo.toml native -p semio-os-store-engine-core -p semio-os-store-engine -p semio-os-store-space-member -p semio-os-store-testing-doubles
