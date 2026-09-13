# Audit: Puzzle 3D Edit-Mode Viewport Render Path — Live Fill-Candidate Ghosts

Editor root `E` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
Host root `H` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

## 1. Data path, Rust `render()` → three.js

**Committed instances** (`Puzzle3dObject`, `E/🦀️.rs:188-211`): `id, label?, objectKind?, origin:[f64;3], orientation?:[f64;4], scale?:DslValue, meshUrl?, vortices:Vec<Puzzle3dVortex>, hidden:bool, locked:bool, revealIndex?:usize (skip_serializing_if none)`. **No color/tint/opacity/emissive field on the object itself** — color is never stored per-instance.

Serialization is hand-rolled `serde_json::json!` text (not the pack container), built by `instance_record_json` (`E/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:210-233`):
```
{ id, meshId, position, rotation, scale, label, disabled: object.locked, objectKind?, revealIndex? }
```
`disabled: object.locked` is the only per-instance visual flag emitted — it maps directly to the host's muted "disabled" mesh style (§1 host side below), so **locked pieces already render immediately as ordinary instances**, just visually muted.

**Diffing**: `Puzzle3dInstanceResidency` (same file, struct at line 317, `refresh()` at 328) is a per-object retained cache keyed by `instance_record_fingerprint` (structural hash of every field `instance_record_json` reads, line 234). `refresh()` rebuilds only changed records, tracks `changed`/`removed` ids, and produces both:
- `instances_json()` — the full authoritative array, always sent (line ~395+, `assemble()`),
- a `delta` (id-keyed changed/removed records + `base`/`revision`) — assembled only `if delta_is_worth_publishing()`.
Host side consumes this via `advanceWorldInstanceResidency` (`H/🌐️World3dHost/🟦️.tsx:1272`), which applies the delta onto a retained `WorldInstanceResidencyV1` when `base` matches, else falls back to full `instancesJson` reparse (`parseInstances`, line 1220).

**Host record type** `WorldInstanceRecord` (`H/🌐️World3dHost/🟦️.tsx:162-186`): `id, meshId?, position?, rotation?, scale?, selected?, hovered?, highlighted?, disabled?, smoothShading?, revealIndex?, objectKind?, interactionId?`. Confirms: **no color/tint/opacity/emissive field exists on the wire type** — paint is 100% derived from boolean state flags via a fixed style table, never authored by the plugin per instance.

**Hover/selection "highlight" mechanism** (`objectKind` field, `🧊️main/🦀️.rs:929` `hovered_kind_id`): when the pointer hovers at `kind` granularity, `world_selection_json` (line ~895) sets `hoveredKindId` from the catalog id; the host then sets `highlighted:true` on every rendered instance whose `objectKind` equals that id (this is the "compatible/suggested" secondary-color state, doc-commented at `WorldInstanceRecord.highlighted`, line 173).

**Mesh style resolution** (`H/🌐️World3dHost/🟦️.tsx`):
- `MeshStyleKind = "disabled" | "celebrated" | "selected" | "highlighted" | "hovered" | "neutral"` (line 451).
- `MESH_STYLE_PAINT` (463-471) hardcodes each kind's `{fill, line, emissiveIntensity, opacity}` using `tokenVar("primary")` / `tokenVar("secondary")` / `semanticVar(...)` / `themeColorVar("muted-foreground")` — **not** `tokenVar("danger")` anywhere.
- `resolveMeshStyle` (line 508) priority: `disabled → celebrated → selected → highlighted → hovered → neutral`. Adding a new kind means inserting it into this priority chain and the paint table.
- `resolveMeshStylePalette()` (474) converts CSS-var expressions to resolved hex via `resolveColorHex`, cached, invalidated on theme flip.

## 2. Per-instance color/tint/state field — does one exist?

**No.** Confirmed at three layers: `Puzzle3dObject` (Rust struct), `instance_record_json` (wire JSON), `WorldInstanceRecord` (TS type). Color for a *committed* instance is never in the instance record at all — the mesh's `color`/`emissive` comes purely from `MESH_STYLE_PAINT[styleKind]`, chosen by the 5 booleans above. Catalog-driven "kind color" (`object_kind_color`, `🦀️.rs:597`, via `catalog_entry_field(meta, "objects", kind_id, ["color"], "#38bdf8")`) is used **only** for: vortex markers, ghost/preview meshes, and popup swatches — never for a placed instance's own material.

**Smallest schema addition** to thread a per-instance render state end-to-end:
1. Rust `Puzzle3dObject`: no change needed for *placed* pieces (locked/hidden already suffice). For the **live candidate ghost**, add a state marker onto the ghost payload instead — see §3/§7.
2. Wire JSON (`instance_record_json` or, better, the ghost JSON emitters in `⏳️precompute/🪣️fill/🦀️.rs`): add `"state": "danger" | "highlight"` (string enum, mirrors the existing `Tone` vocabulary `neutral|primary|secondary|tertiary|info|success|warning|danger`, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs:778`) next to the existing `"color"` field.
3. TS type `WorldBrushPreviewRecord` (`H/🌐️World3dHost/🟦️.tsx:~320-336`, has `opacity?`, `objectKindId?`) — add `state?: "danger" | "highlight"`.
4. `MESH_STYLE_PAINT`: add a `danger` kind → `{ fill: tokenVar("danger"), line: tokenVar("danger"), emissiveIntensity: 0.35, opacity: 0.72 }` (danger token already exists: `--color-danger: #a60009`, `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css:176`; referenced by **string name**, not index, via `tokenVar(key)` → `var(--color-${key})`, `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:407`).
5. `BrushPreviewGhost` (`H/🌐️World3dHost/🟦️.tsx:3667-3691`): change `const style = palette.highlighted;` to pick `palette[preview.state ?? "highlighted"]`, and `meshColor = preview.color ?? style.meshColor` stays (color still comes from the object-kind catalog swatch when present; `state` only picks the *style* — emissive/opacity/fallback tint — not necessarily overriding the kind hue). Simplest correct behavior for "danger when colliding": when `state === "danger"`, force `meshColor = style.meshColor` (the danger token) so the ghost turns red regardless of kind color.

## 3. Existing ghost pattern — brush preview (closest analog)

`🫧️transient` folders under both the mode and this window are **empty placeholders** (`📌️.empty.md` only, no code) — not yet used for this feature.

The **brush** tool's ghost is the reference implementation:
- Rust: `Puzzle3dPrecomputeSession::brush_preview` (`⏳️precompute/🦀️.rs:2993`) looks up a **pre-filtered collision-free candidate** from `brush_cache` (populated by `brush_collision_free`/`brush_collision_free_until`) and calls `brush_preview_from_candidate` (`⏳️precompute/🖌️brush/🦀️.rs:500`) to build a `BrushPreviewState { targetVortexFullId, objectKindId, sourceVortexIndex, meshUrl, origin, orientation, scale }`.
- `world_brush_preview_json` (`🧊️main/🦀️.rs:855`) appends `"color"` from `object_kind_color(meta, kind)` — **the only color source**, always the catalog swatch, never collision-state-dependent (brush candidates are collision-filtered *before* becoming a preview, so a "danger" brush ghost cannot occur today).
- TS: `parseWorldBrushPreview` (line 1641) validates and decodes `brushPreviewJson`; `BrushPreviewGhost` (line 3667) renders it with `palette.highlighted`, `emissiveIntensity=0.6`, `opacity=0.72` (GLB) or a fallback box at `opacity=0.42`.

**Fill's ghost reuses this exact component.** `world_fill_preview_json` (`🧊️main/🦀️.rs:865-871`) calls `session.fill_preview_json_page(color, status_label)`, feeding the SAME `brush_preview_json` scene slot (`render()`, line ~965: `let brush_preview = world_fill_preview_json(...).or_else(|| world_brush_preview_json(...))`). Host-side, `visibleBrushPreview = fillMode ? (fillDiagnostic?.candidateGhost ? brushPreview : null) : brushPreview` (`H/🌐️World3dHost/🟦️.tsx:5306`) — so **the fill tool's "currently tested candidate" ghost is rendered by the same `BrushPreviewGhost`, same fixed `highlighted` style, same hardcoded opacity**. There is currently no visual distinction between "still searching," "about to be accepted," or "just rejected for collision."

## 4. Fill build state machine — where collision/danger state actually lives

`⏳️precompute/🪣️fill/🦀️.rs`, `FillJobStage` enum (line 868) drives one candidate at a time through: `SelectTarget → PrepareCandidates → SelectCandidate → ConstructPreview → QueryBroadPhase → TestCollision → AcceptCandidate` (or `reject_candidate("solid-overlap")` back to `SelectCandidate`, `reject_target(...)` back to `SelectTarget`).
- `test_collision` (line 3921): on `CollisionStepResult::Complete { overlap, .. } if overlap > self.overlap_budget` → `self.preview.collision_count += 1; self.reject_candidate("solid-overlap")`. This is the **exact "colliding" moment** — but the code rejects and advances to the next candidate in the same step, so a colliding ghost is only visible for the fraction of a tick before the cache moves on (the retained "last valid page" logic in `fill_preview_json_page`, `⏳️precompute/🦀️.rs:3024`, means a consumer sees the ghost update once per completed unit, not per raw collision check).
- `preview.rejection_reason: Option<String>` (set in `reject_candidate`/`reject_target`) and `preview.stage: String` (`stage_label`, `🪣️fill/🦀️.rs:4268+`) are already serialized into the ghost's diagnostic block (`fillBuildPreview.rejectionReason`, `.stage`) — this is the natural boolean to key `state: "danger"` off: **`rejection_reason == Some("solid-overlap")` at publish time ⇒ danger; otherwise (mid-test, or about to accept) ⇒ highlight**.
- `accept_candidate` (line 4160-4200): on success, `placed_object.reveal_index = Some(self.appended_objects.len())`, pushed to `appended_objects`/`sequence` — this is how an accepted candidate becomes a normal (reveal-tagged) instance.

## 5. Reveal mechanism

`WindowMeasure::Slider.reveal: Option<String>` (canonical struct `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1081`). Puzzle3d's fill-count slider sets `reveal: Some("puzzle3d-fill".into())` (`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:44`).

Client-side cutoff store (`H/🛠️ShellHelpers/🟦️.tsx:2778-2855`):
- `RevealCutoffStore` (2786) — plain `Map<groupId, number>` + pub/sub, **main-thread only, never dispatched to WASM**.
- `PUZZLE3D_FILL_REVEAL_GROUP_ID = "puzzle3d-fill"` (2819) — the only reveal group that exists today.
- `worldRevealCutoffStore` (2816) — module-singleton instance.
- `reconcileCommittedRevealCutoffs` (2825) — writes the plugin's committed `interactionJson.revealCutoffs["puzzle3d-fill"]` (`🧊️main/🦀️.rs:791`, `runtime.fill_count`) into the store, but **skips a no-op value change** so it never clobbers a live slider drag.
- `isRevealCutoffHidden` (2848) — pure predicate: `instance.revealIndex != null && instance.revealIndex >= cutoff`.
- `WorldInstancesLayer` (`H/🌐️World3dHost/🟦️.tsx:2921`, the imperative-hide effect at line 3075-3092) walks `instances`, toggles `Object3D.visible` per `revealIndex` vs. cutoff — **zero React re-render, zero WASM round trip** for a slider drag.

For "truly incremental fill," this whole cutoff layer becomes unnecessary if accepted candidates are revealed the instant they're accepted (reveal_index assigned but cutoff always ≥ current count) rather than batch-planned-then-slid-open — i.e. the mechanism to remove/bypass is `revealIndex`/`revealCutoffs`/`RevealCutoffStore`, replaced by ordinary immediate instance publication (§7).

## 6. Live progress readouts

Two independent readouts exist today, both fed by `FillProgressSummary`/`FillBuildPreview`:
- **Viewport HUD**: `FillDiagnosticOverlay` (`H/🌐️World3dHost/🟦️.tsx:3695-3740`) — a fixed `bottom-3 left-3` glass panel, `role="status"`, rendered whenever `fillDiagnostic` (i.e. `brushPreview.fillBuildPreview`) is present. Shows `statusLabel`, `stage`, `acceptedCount/totalCount`, `rejectionReason ?? collisionCount`, truncation marker. All fields also land as `data-fill-*` attributes for tests. **This is the closest existing thing to "tested N / locked M / requested K"** — it already has `acceptedCount`/`totalCount`; `rejected_count`/`collision_count` are present in the Rust struct but not yet surfaced as a labeled "tested" number in this overlay's visible text (only via `rejectionReason ?? String(collisionCount)`).
- **Panel row**: `cancel_measure` (`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:52-64`) — a `WindowMeasure::Toggle` with `text: Some(format!("{count} / {max_count} {planned}"))`, shown only while `!progress.done`, with a real cancel button wired to `Effect::CancelJob` (via `cancelFillBuild` action, carrying `(job, operation, generation)` identity so a stale cancel is a no-op).
- **Notices** (`ctx.notice`, `E/🦀️.rs:2725`): pushes `Effect::Notify { message }`, **deduplicated to at most one per dispatch** (`if self.effects.iter().any(|e| matches!(e, Effect::Notify{..})) { return; }`). This is a one-shot toast (used today only for `fill_failed`, `nothing_selected`, `selection_locked`) — **not** appropriate for a continuously-updating "tested 37/100" counter; that belongs in the HUD/panel text above, refreshed every tick.

## 7. Performance constraints

- `UI_TEXT_MAX_BYTES = 512` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:18`), `UiText::clipped` truncates with a suffix — relevant if a progress string is threaded through any `UiText`-typed action-arg/label field (it is not currently — `Toggle.text` in `cancel_measure` is a plain `String`/label, and diagnostic JSON strings are separately capped, see next line).
- Ghost/diagnostic JSON payload caps (`H/🌐️World3dHost/🟦️.tsx:366-368`): `WORLD_FILL_STATUS_LABEL_MAX_BYTES=256`, `WORLD_FILL_COLOR_MAX_BYTES=128`, `WORLD_FILL_PREVIEW_JSON_MAX_BYTES=4096` — any new `state` field must fit comfortably inside the existing 4 KiB ghost JSON envelope (trivial: `"state":"danger"` is ~16 bytes).
- Suggestion popup page size: `PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE = 8` (`🧊️main/🦀️.rs:~640`) — bounded so an unbounded collision-free candidate list can't blow the fixed-capacity scene surface.
- `world3d_chunking_json(envelope.runtime.chunk_size, 8000.0)` (`🧊️main/🦀️.rs`, `render()`) — 8000-unit chunk/LOD budget for the whole scene, unrelated to instance *count* per se but caps spatial partition granularity.
- `fill_preview_json_page` (`⏳️precompute/🦀️.rs:3024`) throttles to `GRANTS_PER_FRAME=256` byte-grants within a `2_000µs` (2ms) deadline per call, and **retains the last valid page** while a newer generation is still encoding — so the ghost/diagnostic the viewport shows is always a consistent (if slightly stale) full JSON object, never a half-written one.
- Re-render cadence (`UiDirtyScope`, `E/🦀️.rs:2204-2276`, doc comment at 2211): background fill polling (`fill-build-tick`, `E/🎮️commands/🪣️fill-build-tick/🦀️.rs`) fires on a **120ms host tick** (comment at `E/🎮️commands/⏱️suggestions-tick/🦀️.rs:8`, shared cadence) and emits `puzzle3d_fill_build_scope()` — `Partial{ window_bodies:[main::BODY_KEY], tools:true, everything else false }` — i.e. **only the one World3d body + fill-tool measures repaint per tick**, never panels/chrome/labels. The doc comment explicitly records the regression this guards against: "Emitting `Full` on every 120ms tick was half of the fill-utility stall" — any new per-tick field (candidate ghost state, tested/rejected counters) MUST stay inside this same narrow `FillBuild` scope, not widen it.
- Instance-level cost control is per-object structural hashing (`instance_record_fingerprint`), not per-frame full-document re-serialization — measured saving from wave B44: 20716µs→near-zero for a no-op render on a 180-object document (comment at `🧊️main/🦀️.rs` `Puzzle3dInstanceResidency` doc, ~line 300).

## 8. Concrete recommendation

**(a) Live ghost, danger/highlight by collision state — smallest correct change:**
- Rust, `⏳️precompute/🪣️fill/🦀️.rs`: in the `FillPreviewJsonPass::next_unit`/`candidate_ghost` emitter (~line 340-480), add one more quoted field after `"color"` (field index 7, line ~471): `"state"`, sourced from a new `FillPreviewString::State` variant whose `string()` arm (line ~303) returns `"danger"` when `preview.rejection_reason.as_deref() == Some("solid-overlap")` else `"highlight"`. Bump every subsequent field index in the `next_unit` match by one (mechanical, this is a hand-rolled streaming JSON writer with numbered states — check `FillPreviewJsonUnit` byte-budget assumptions in the file's tests, `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`, when doing this).
- Also thread it onto the **brush** ghost path (`brush_preview_from_candidate`, `⏳️precompute/🖌️brush/🦀️.rs:500`) as `state: "highlight"` unconditionally (brush candidates are pre-filtered collision-free, so brush never needs danger — but the shared TS record/component should treat its absence as `"highlight"` default so brush needs no change at all).
- TS, `H/🌐️World3dHost/🟦️.tsx`:
  - `WorldBrushPreviewRecord` type (~line 320): add `state?: "danger" | "highlight"`, with `WORLD_FILL_GHOST_KEYS`/`WORLD_FILL_ROOT_KEYS` (lines 369, 396) census sets extended to allow it, and the manual validator around line 1667-1673 extended to check `state === undefined || state === "danger" || state === "highlight"`.
  - `MESH_STYLE_PAINT` (463): add `danger: { fill: tokenVar("danger"), line: tokenVar("danger"), emissiveIntensity: 0.35, opacity: 0.72 }`.
  - `BrushPreviewGhost` (3667): `const style = palette[preview.state === "danger" ? "danger" : "highlighted"]; const meshColor = preview.state === "danger" ? style.meshColor : (preview.color ?? style.meshColor);` — danger always shows the token red (ignore catalog kind color so the safety signal is unambiguous), highlight keeps today's kind-color behavior.
  - `FillDiagnosticOverlay`/`fillDiagnostic` typing (`WorldFillDiagnosticRecord.candidateGhost`, line 349) inherits the new field automatically since it types `candidateGhost: WorldBrushPreviewRecord | null`.

**(b) Recently rejected candidates fading (optional):** not free — `reset_collision(true)` (`⏳️precompute/🪣️fill/🦀️.rs`, called from `reject_target`) clears `preview.candidate_ghost` immediately on rejection, so the rejected pose is gone by the next publish. To fade it, keep a short-lived `last_rejected_ghost: Option<(FillBuildPreview, u64 /* tick */)>` in `FillBuilder`, serialize it as a second ghost slot (`"rejectedGhost"`) alongside `candidateGhost`, decay its opacity client-side over N ticks (a simple `Date.now()`-keyed fade in `BrushPreviewGhost`'s sibling). Given the 120ms tick and the fact collisions reject-and-advance same-step, this is genuinely optional polish, not core to "visible process."

**(c) Locked pieces as normal instances immediately:** already true today — `instance_record_json` emits every `Puzzle3dObject` (locked or not) with `disabled: object.locked`; nothing to build. If the intent is "locked pieces should look *complete/normal*, not muted-gray," that's a product decision to stop mapping `locked → disabled` style (drop that line in `instance_record_json`, or introduce a distinct `locked` boolean on `WorldInstanceRecord`/`resolveMeshStyle` so locked ≠ visually-disabled).

**(d) Progress readout:** extend `FillProgressSummary`/`fill_progress_summary()` (`⏳️precompute/🦀️.rs:3011`) — it already tracks `count`/`applied_count`/`max_count`/`done`; add `rejected_count`/`tested_count` (both already tracked internally on `FillBuilder` as `self.rejected_count`/increment counters near `reject_candidate`/`reject_target`, `⏳️precompute/🪣️fill/🦀️.rs:4150+`) to the summary struct, thread into `world_interaction_json`'s `"fillBuild"` block (`🧊️main/🦀️.rs:774-791`, currently `{count, appliedCount, maxCount, done}` → add `rejectedCount`/`testedCount`), and either (i) extend `FillDiagnosticOverlay`'s visible `<span>` list (line ~3732-3738) with an explicit "tested N / locked M / requested K" string built from `acceptedCount`/`totalCount`/`rejectedCount`, or (ii) extend `cancel_measure`'s panel text (`E/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:60`) similarly. Keep this inside the existing `puzzle3d_fill_build_scope()` dirty-scope (§7) so it doesn't regress the 120ms-tick performance fix.

## Files touched by this recommendation (none edited by this audit — read-only)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` (ghost `state` field, progress counters)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs` (`FillProgressSummary`, `fill_progress_summary`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (`world_interaction_json` fillBuild block)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (`cancel_measure` text)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` (`WorldBrushPreviewRecord`, `MESH_STYLE_PAINT`, `BrushPreviewGhost`, `FillDiagnosticOverlay`, census/validator sets)
