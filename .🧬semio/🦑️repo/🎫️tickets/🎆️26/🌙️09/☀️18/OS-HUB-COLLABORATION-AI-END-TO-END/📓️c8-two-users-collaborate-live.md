# C8 — two humans, two browsers, ONE hub document

Slice C8 (session 8, 2026-09-22). Inherits C7 §0/§3: the mount chain's defects 1–4 are fixed and
proven at runtime; defect 5 (the guest refusing the GENESIS cold-pair frontier) is fixed in source
and needs the gis guest rebuilt into the actor the browser actually runs.

## 0. HANDOFF

| field | value |
|---|---|
| document MOUNTS | (filling) |
| scenario steps green | (filling) |
| gate wired | (filling) |
| infra | (filling) |

## 1. Where the guest comes from — the correction to C7's resume

**C7's resume step 1 ("re-activate the `gis2d` dev variant and the guest fix takes effect") cannot
work, and this is measured, not argued.** The browser's document actor is not the dev-serve
component: it is `packages/gis/browser/closed-actor.mjs` of the **hub's published catalog
generation**.

| reading | value | where |
|---|---|---|
| actor core the browser decodes, baseline run on 7621 | **47 416 521 B** | `🗑️generated/c8a-actor-probe.txt`, stage `actor-decode …/47416521` |
| the only file of that size on disk | `.🧬semio/🌐hub/jc1-boot/trusted-catalog/generations/8086b61f…/packages/gis/browser/closed-actor.mjs` (63 717 043 B, **2026-09-21 05:33**) | `find` |
| the gis component the dev variant would restage | `✏️s/🔌️plugins/🌍️gis/…/dist/component-dev/semio_s_plugin_gis.wasm`, 215 377 769 B, 2026-09-22 02:39 | a different artifact entirely |
| C7's guest fix | `🎭️actor/📥️cold-pair/🦀️.rs`, `🎠️kernel/📥️cold-pair/🦀️.rs`, both **2026-09-21 14:26** | later than the catalog generation |

So the running guest is the 05:33 build, **8 h 53 min older than the fix**, and no `activate gis2d
react dev` run touches it. C8's baseline probe reproduces C7's defect 5 exactly and with C8's own
capture:

```
DIAGNOSTIC document browser actor: invalid page receipt (page 1/2 answered fault page 1/2: cold-pair.frontier)
ACTIVE false (verifying=true rendererUnavailableSeen=false cleared=false canvas=1587x907)
STAGE   63552ms canonical-pair 0/1
STAGE   65065ms actor-decode 147456/47416521
```

(`c8a`, hub 7621, serve 6191 — the `canonical-pair` stage is C7's defect-1 fix working.)

The consequence is the shape of this slice: **the guest fix reaches a browser only through a catalog
republish**, and the running hub 7621 may be neither restarted nor republished into (a peer shares
it, and its binary is executing in place — an in-place overwrite is a silent SIGKILL on macOS).

### 1.1 A second, independent reason to leave 7621

Task brief: *"PR1's fixes are in the 01:53 hub binary — verify"*. **They are not.** Hub 7621 (pid
607) executes `⚡️cache/cargo/target-jc1/debug/os-hub`, built **2026-09-21 03:42**; PR1 landed its
hub half later that day:

```
nm -a target-jc1/debug/os-hub    | grep -c presence_replay  → 0
nm -a target-c8-hub/debug/os-hub | grep -c presence_replay  → 4
```

`subscribe_with_presence_replay` is PR1 §8's join-replay entry point. Step 5 (symmetric rosters) is
therefore **unmeasurable on 7621** — the join replay it tests is not in that process. The same
binary also predates `agent_delegation_ready`, so 7621 publishes `features.mcpWorkspace: false`
(`curl :7621/readyz`), which is step 10's own gate.

## 2. The republish — a second hub on 7671 from C8's own catalog

(filling)

## 3. The single-context gate

(filling)

## 4. The ten steps

| # | step | verdict | number | capture |
|---|---|---|---|---|
| 1 | two sockets on one document | (filling) | | |
| 2 | live edit A→B | (filling) | | |
| 3 | live edit B→A | (filling) | | |
| 4 | per-user undo against the peer's write | (filling) | | |
| 5 | presence colours symmetric on BOTH rosters | (filling) | | |
| 6 | short connection loss + reconvergence | (filling) | | |
| 7 | two simultaneous writers converge | (filling) | | |
| 8 | reload re-attach | (filling) | | |
| 9 | mid-edit hub restart | (filling) | | |
| 10 | agent third participant | (filling) | | |

## 5. Product defects found and fixed

(filling)

## 6. Honest gaps

(filling)

## 7. Files changed and captures

(filling)
