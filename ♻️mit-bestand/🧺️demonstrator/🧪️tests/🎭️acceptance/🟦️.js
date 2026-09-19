"use strict";
// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts
// Specs: End-to-end acceptance coverage for the "Entwerfen mit Bestand" demonstrator's eight live panes.
// Summary: For every `DEMONSTRATOR_PANES` entry, deep-links straight to `/#<paneId>` (the fast path that
// boots exactly that one pane immediately instead of waiting through the 1.5s/35s sequential-boot queue —
// see `🟦️.tsx`'s `paneIdFromLocationHash`/`useSequentialPaneBoot`), waits for that pane's own
// `FrameworkOsShell` to report readiness via the per-shell `data-shell-ready`/`data-shell-error` beacon
// (`ShellHost/🟦️.tsx`'s `#region 🔖️ReadinessBeacon`), asserts its declared window(s) attach, and
// asserts each window actually carries rendered content (not an empty surface, not the "wird vorbereitet"
// `CanvasSkeleton`) by reading the same production `data-*-json`/`data-row-id` attributes each surface host
// already stamps on itself (`World3dHost`'s `data-meshes-json`/`data-instances-json`, `NodeGraph`'s
// `data-host-snapshot-json`, `Table`'s `data-row-id` rows, and — for the two energy windows whose body is
// a plain framework `Tree` rather than a surface host — the `role="treeitem"` rows `🌳️Tree/🟦️.tsx` emits)
// — no test-only instrumentation was added for
// this. `TiledMapHost` is the one surface with no such attribute, so it is graded from a composited
// element screenshot instead (see `tiledMapHasVisibleContent` for why a canvas readback cannot work).
// Known-defect windows are asserted exactly like every other window rather than being weakened or
// skipped, so a real defect fails loudly. Generator's edit-mode preview used to be expected to fail (it
// never got a synced flow-eval session — `📓️app-generator.md` §3); the guest half of that is fixed and
// proven natively (`📓️fix-2026-09-16-generator-eval-session.md`), so it now grades the served path only.
// Koordinator's four CAD windows were
// the same story until commit f394df99d4 wired `cad_pane_working_scene` through
// `ArtifactChild::local_owner`; that fix is committed but NOT yet compile-verified, so these four
// assertions are its first real proof — see `📓️app-koordinator.md`. Every test also fails on any page error or non-404 console error, following
// `.storybook/framework-hosts-wasm.spec.ts`'s `expectHostStory` idiom.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
// #region 🔌️Adapters
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var test_1 = require("@playwright/test");
// #endregion 🔌️Adapters
//#region 🪪️BrandPaneIds
/** @emoji 🪪️ Reads `🪧️brand.ts`'s pane ids as TEXT rather than importing it. Playwright loads specs
 * through Node's own ESM loader, and importing the brand module drags in the whole `@semio-tech/ui-react`
 * runtime — whose typed-scene catalog is a bare `.json` import that Node rejects without an
 * `import ... with { type: "json" }` attribute (`ERR_IMPORT_ATTRIBUTE_MISSING`). Vite rewrites that for the
 * app; Playwright does not, so this suite stays import-free of app modules exactly like `.storybook`'s specs. */
function brandPaneIds() {
    var source = (0, node_fs_1.readFileSync)((0, node_path_1.join)(import.meta.dirname, "..", "..", "🪧️brand.ts"), "utf8");
    var block = /export const DEMONSTRATOR_PANES[^=]*=\s*\[(.*?)\n\];/s.exec(source);
    if (!block)
        throw new Error("🪧️brand.ts no longer declares a DEMONSTRATOR_PANES array literal");
    return __spreadArray([], block[1].matchAll(/\bid:\s*"([^"]+)"/g), true).map(function (match) { return match[1]; });
}
//#endregion 🪪️BrandPaneIds
/** @emoji ⏱️ Cold WASM plugin boots on a fresh page load can be slow (memory: "expect slow cold boots, be
 * patient") — generous on purpose so this suite reports real content defects, not infra flakiness. */
var SHELL_READY_TIMEOUT_MS = 120000;
var TEST_TIMEOUT_MS = 240000;
/** @emoji ⏳️ How long a window that IS expected to carry content may take to publish its first non-empty
 * frame after its surface host attaches — the shell reports "ready" as soon as the plugin's UI tree is
 * mounted, which is strictly before the guest's first scene/evaluation crosses the wire. */
var SURFACE_CONTENT_TIMEOUT_MS = 60000;
/** 🔇️ Drops resource 404s and the repo's `[DEBUG] `-prefixed temporary diagnostics, which hosts emit at error level while a seam is being instrumented. */
function significantConsoleErrors(messages) {
    return messages.filter(function (text) { return !/Failed to load resource:.*\b40[0-9]\b/i.test(text) && !text.startsWith("[DEBUG] "); });
}
//#region 🆔️ElementId
/** @emoji 🆔️ Local mirror of `framework/ui/elements/🆔️ElementId/🟦️.tsx`'s `elementIdSegment` — kept
 * as a tiny pure copy rather than importing the framework's React-bearing module into a Playwright spec. */
function elementIdSegment(raw) {
    var segment = "";
    var capitalizeNext = false;
    for (var _i = 0, raw_1 = raw; _i < raw_1.length; _i++) {
        var ch = raw_1[_i];
        if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
            capitalizeNext = true;
            continue;
        }
        if (!/[a-zA-Z0-9]/.test(ch))
            continue;
        if (segment.length === 0)
            segment += ch.toLowerCase();
        else if (capitalizeNext) {
            segment += ch.toUpperCase();
            capitalizeNext = false;
        }
        else
            segment += ch;
    }
    return segment;
}
/** @emoji 🪟️ Mirrors `framework/platform`'s `windowElementId(kindId)` → `"framework.window.<camelKindId>"`. */
function windowElementId(kindId) {
    return "framework.window.".concat(elementIdSegment(kindId));
}
/** @emoji 🎯️ CSS selector for the element carrying `id` as either its real DOM id or a `data-element-alias`
 * token, scoped to one pane's shell root — mirrors `elementIdSelector`, scoped by `[data-shell-id]`. */
function paneElementSelector(paneId, elementId) {
    return "[data-shell-id=\"".concat(paneId, "\"] [id=\"").concat(elementId, "\"], [data-shell-id=\"").concat(paneId, "\"] [data-element-alias~=\"").concat(elementId, "\"]");
}
/** @emoji 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome via the per-shell
 * `data-shell-ready`/`data-shell-error`/`data-shell-not-found` beacon (added alongside the pre-existing
 * global `document.documentElement` one specifically so a page hosting several shells can ask "is THIS
 * one ready" — see `ShellHost/🟦️.tsx`'s `#region 🔖️ReadinessBeacon`). */
function waitForPaneShellOutcome(page, paneId) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, page.waitForFunction(function (id) {
                        var el = document.querySelector("[data-shell-id=\"".concat(id, "\"]"));
                        return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
                    }, paneId, { timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    return [2 /*return*/, page.evaluate(function (id) {
                            var el = document.querySelector("[data-shell-id=\"".concat(id, "\"]"));
                            if (el.dataset.shellReady !== undefined)
                                return "ready";
                            if (el.dataset.shellError !== undefined)
                                return "error";
                            return "notFound";
                        }, paneId)];
            }
        });
    });
}
/** @emoji 👋️ Dismisses an introduction overlay, by pointer first and by its own documented keyboard
 * parity route second.
 *
 * `ui.introduction.skip` anchors itself to the surface the current tour step highlights
 * (`registerIntroductionSurfaceResolver`), so while a cold page is still booting shells the box
 * re-anchors and the button's box moves — Playwright's actionability "stable" gate then never settles
 * and a bare `click()` times out mid-boot even though the button is visible and enabled (measured
 * 2026-09-16: on a warm server the box holds for 300 straight frames and the click lands in ~230 ms;
 * on the cold acceptance server run 1 the same click never became stable inside 5 s). So: one generous
 * click, then `escape` — the keybinding this very control declares
 * (`🕹️control-keybinding-context/🟦️.tsx`'s `"ui.introduction.skip": "escape"`), which needs no
 * actionability at all — and finally proof that the overlay really went away. */
function dismissIntroduction(page, skip) {
    return __awaiter(this, void 0, void 0, function () {
        var clicked;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, skip.first().isVisible({ timeout: 3000 }).catch(function () { return false; })];
                case 1:
                    if (!(_a.sent()))
                        return [2 /*return*/];
                    return [4 /*yield*/, skip
                            .first()
                            .click({ timeout: 20000 })
                            .then(function () { return true; })
                            .catch(function () { return false; })];
                case 2:
                    clicked = _a.sent();
                    if (!!clicked) return [3 /*break*/, 4];
                    return [4 /*yield*/, page.keyboard.press("Escape")];
                case 3:
                    _a.sent();
                    _a.label = 4;
                case 4: return [4 /*yield*/, (0, test_1.expect)(skip, "the introduction overlay must be dismissable").toHaveCount(0, { timeout: 15000 })];
                case 5:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
/** @emoji 👋️ Every brand replays its own app-level introduction on load once focused
 * (`suppressAutoIntroduction={!focused}` is false for a hash-deep-linked, already-focused pane) — dismiss
 * it so it never shadows a later interaction. Absence is not an error (already dismissed, or this brand
 * has none left to show). */
function dismissIntroductionIfPresent(page, paneId) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, dismissIntroduction(page, page.locator(paneElementSelector(paneId, "ui.introduction.skip")))];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
var PANE_CASES = [
    {
        paneId: "generator",
        windows: [
            { kindId: "procedural-main", surface: "nodeGraph", expectContent: true },
            {
                kindId: "procedural-preview",
                surface: "world3d",
                expectContent: true,
                note: "Was a KNOWN GAP (📓️app-generator.md §3): edit-mode render() built a fresh, never-ticked FlowEvalSession per call. The session is retained per app instance now and the evaluation is the progress/cancel-capable `previewEval` tool run; the guest half is proven natively by `the_demonstrator_boot_example_renders_a_non_empty_preview_scene` (generation3d edit preview window unit tests, 📓️fix-2026-09-16-generator-eval-session.md). This assertion therefore grades the SERVED path only.",
            },
        ],
    },
    {
        paneId: "koordinator",
        windows: [
            {
                kindId: "cad-play-shape",
                surface: "world3d",
                expectContent: true,
                note: "Was a KNOWN GAP: `build_world_scene_for_pane` hardcoded an empty `&[]` object slice. FIXED in commit f394df99d4 — `cad_pane_working_scene` now resolves the pane's composed child through `ArtifactChild::local_owner`, and `forest_play_document` no longer discards its fixture JSON. NOT yet compile-verified (`semio-s-plugin-cad` is blocked by peer `🏪️store` E0119 errors), so this assertion is the first real proof of that fix.",
            },
            { kindId: "cad-play-building", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
            { kindId: "cad-play-energy", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
            { kindId: "cad-play-structure-classic", surface: "world3d", expectContent: true, note: "same shared render boundary as cad-play-shape — covered by the same f394df99d4 fix." },
        ],
    },
    {
        paneId: "aggregator",
        windows: [{ kindId: "puzzle3d-main", instanceIds: ["puzzle3d-main-top", "puzzle3d-main-perspective"], surface: "world3d", expectContent: true }],
    },
    {
        paneId: "energie",
        windows: [
            {
                kindId: "energy.model.3d",
                surface: "world3d",
                expectContent: true,
                note: "`s.energy.model@1/*#editor`'s Model viewport — `✏️editor/🎭️modes/✏️edit/🦀️.rs`'s `layout()` gives it `MODEL_VIEWPORT_SHARE` (0.55) of the edit-mode row, with Structure/Zones/Energy simulation stacked in the remaining column. Measured 2026-09-17 on the live demonstrator serve: `/#energie` reached `ready` in 11.5 s and all four windows carried content by 15.3 s, with no page or console errors; this window read 9 meshes / 9 instances. The brand's `defaults.exampleId` is `bestest-600`, whose `artifactJson` is byte-identical to the plugin's `demo` example, so the standalone energy lane (:6106) grades the same document.",
            },
            {
                kindId: "framework.window.tree",
                surface: "tree",
                expectContent: true,
                note: "Structure. The energy editor reuses the framework's generic `TreeWindowKit` kind id verbatim, so the window element is `framework.window.frameworkWindowTree` (`elementIdSegment` folds the dots) — the pane-scoped selector keeps that generic id from colliding with any other shell on the page. The body is a plain framework `Tree`, NOT a surface host, so there is no `.semio-*-host` to grade; measured on :6106 it carries 46 `role=\"treeitem\"` rows headed `BESTEST 600 (vashrae-140-5.2)`. NOTE: the committed `✏️s/🔌️plugins/🔋️energy/🔣️.json` still declares this kind as `surfaceKind: \"block-list\"` and omits `energy.model.3d` entirely — that descriptor is stale relative to the editor source and the served build; the live DOM is what this expectation is written from.",
            },
            {
                kindId: "framework.window.table",
                surface: "table",
                expectContent: true,
                note: "Zones — the generic `TableWindowKit` kind id, so the element is `framework.window.frameworkWindowTable`. BESTEST 600 is a single-zone model, so exactly one `[data-row-id]` row (`id/name/volumeM3/multiplier/conditioned/partOfTotalFloorArea` → `1 / Zone / 129.6 / 1 / true / true`) is the correct, fully-loaded content — the assertion floor of one row is deliberately not raised.",
            },
            {
                kindId: "energy.simulation",
                surface: "tree",
                expectContent: true,
                note: "Energy simulation. Also a framework `Tree` body (run transport, live region, editable run settings, keybinding list) rather than a surface host: 15 `role=\"treeitem\"` rows on a fresh document, headed `Kein Energiesimulationslauf` under this brand's locked German locale. This grades that the simulation window's projection rendered at all — it deliberately does NOT require a completed run, which is a user-started tool run (`toolRun*` on `energySimulation`), not part of boot.",
            },
        ],
    },
    {
        paneId: "aussuchen",
        windows: [
            { kindId: "sourcing-pool", surface: "table", expectContent: true },
            { kindId: "sourcing-curated", surface: "table", expectContent: false, note: "Curated starts empty by design — nothing has been curated yet on a fresh document." },
            {
                kindId: "sourcing-preview",
                surface: "placeholder",
                expectContent: false,
                placeholderPattern: /Keine Auswahl|No selection/,
                note: "Declared `SurfaceKind::World3d`, but on a fresh document nothing is selected and `👁️preview/🦀️.rs`'s `render` deliberately returns `built_text_node(labels.no_selection)` — a text body, so there is no `.semio-world-3d-host`/`.semio-world-3d-empty` to grade at all. The old `📓️app-aussuchen.md §5` gap (`preview::render(snapshot, &[], labels)` hardcoded at the interaction-less delegate) is CLOSED: `✏️editor/🦀️.rs:1091` now feeds it `interaction.selection(SOURCING_ROWS_DOMAIN)`, and the scene that produces is graded by the aussuchen selection test below.",
            },
            {
                kindId: "sourcing-grid",
                surface: "world3d",
                expectContent: false,
                note: "Grid lays out the curated set only; on a fresh document curation is empty (same as sourcing-curated).",
            },
        ],
    },
    { paneId: "bearbeiten", windows: [{ kindId: "process-workpiece", surface: "world3d", expectContent: true }] },
    { paneId: "verfolgen", windows: [{ kindId: "gis2d-main", surface: "tiledMap", expectContent: true }] },
    {
        paneId: "statik",
        windows: [
            {
                kindId: "fem3d-model",
                surface: "world3d",
                expectContent: true,
                note: "`s.fem.fem3d@1/*#editor` opens exactly two windows, Model and Results, split 50/50 by its default layout (`🪟️windows/🧱️model` + `📊️results`, both `SurfaceKind::World3d`). Measured 2026-09-17 on the live demonstrator serve: `/#statik` reached `ready` in 4.6 s and both windows carried 3 meshes / 47 instances at 6.0 s, with no page or console errors. On the standalone fem3d lane (:6087) the `demo` example reads the same 3/47 while `concrete-forest` — the id this pane's brand sets as `defaults.exampleId` — reads 2 meshes / 234 instances, so the served pane looks like it is still booting `demo`; that is a brand/default question for the coordinator, not a reason to weaken this assertion, which only requires a non-empty scene and passes on either example.",
            },
            {
                kindId: "fem3d-results",
                surface: "world3d",
                expectContent: true,
                note: "Results draws the same solid set as Model until a solve lands and then recolours it, so it is non-empty from the first frame and is graded exactly like Model (identical counts measured in both windows, plus a `Case: dead` caption). The expensive path here is the SOLVE, not the boot: `📓️fem3d-interactive-2026-09-16.md` clocks the House example at ~11 s in wasm. Neither this assertion nor the Model one waits on a solve, and the whole pane was measured ready+filled in 6 s on the live serve, so `SURFACE_CONTENT_TIMEOUT_MS`/`TEST_TIMEOUT_MS` need no per-pane relaxation the way verfolgen's slow tile paint does.",
            },
        ],
    },
];
/** @emoji 📖️ Parses a `data-*-json` attribute into its array length, treating a missing/unparsable
 * attribute as zero rather than throwing — an absent attribute is itself evidence of "no content yet". */
function jsonArrayLength(raw) {
    if (!raw)
        return 0;
    try {
        var parsed = JSON.parse(raw);
        return Array.isArray(parsed) ? parsed.length : 0;
    }
    catch (_a) {
        return 0;
    }
}
/** @emoji ⏳️ A surface host attaches EMPTY and is filled by the first frame the guest publishes across
 * the wire, so a single sample right after the host becomes visible grades the boot gap, not the app: on
 * the 2026-09-16 serve generator's preview read `meshes=0` at 2.8 s and a 3689-byte `data-meshes-json`
 * once its `previewEval` run landed. This re-reads until the count is non-zero — it only ever waits on
 * the windows that are *expected* to carry content, so a genuinely empty surface still fails, just
 * `SURFACE_CONTENT_TIMEOUT_MS` later instead of instantly. */
function settleContentCount(read_1, requireContent_1) {
    return __awaiter(this, arguments, void 0, function (read, requireContent, minimum) {
        var value, deadline;
        if (minimum === void 0) { minimum = 1; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, read()];
                case 1:
                    value = _a.sent();
                    deadline = Date.now() + SURFACE_CONTENT_TIMEOUT_MS;
                    _a.label = 2;
                case 2:
                    if (!(requireContent && value < minimum && Date.now() < deadline)) return [3 /*break*/, 5];
                    return [4 /*yield*/, new Promise(function (resolve) { return setTimeout(resolve, 500); })];
                case 3:
                    _a.sent();
                    return [4 /*yield*/, read()];
                case 4:
                    value = _a.sent();
                    return [3 /*break*/, 2];
                case 5: return [2 /*return*/, value];
            }
        });
    });
}
/** @emoji 🌍️ `World3dHost` stamps its live scene straight onto `.semio-world-3d-host` as
 * `data-meshes-json`/`data-instances-json` (`World3dHost/🟦️.tsx` ~line 5063) — reading those is
 * strictly more reliable than sampling canvas pixels (no readback-timing/`preserveDrawingBuffer` gotchas). */
function worldContentCount(container_1) {
    return __awaiter(this, arguments, void 0, function (container, requireContent) {
        var host, empty, readAttrs, counts;
        var _this = this;
        if (requireContent === void 0) { requireContent = false; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    host = container.locator(".semio-world-3d-host");
                    empty = container.locator(".semio-world-3d-empty");
                    return [4 /*yield*/, (0, test_1.expect)(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    if (!requireContent) return [3 /*break*/, 3];
                    return [4 /*yield*/, host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(function () { })];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3: return [4 /*yield*/, host.count()];
                case 4:
                    if ((_a.sent()) === 0)
                        return [2 /*return*/, { hasScene: false, meshes: 0, instances: 0 }];
                    readAttrs = function () { return __awaiter(_this, void 0, void 0, function () {
                        var attrs;
                        return __generator(this, function (_a) {
                            switch (_a.label) {
                                case 0: return [4 /*yield*/, host.evaluate(function (el) { return ({ meshes: el.getAttribute("data-meshes-json"), instances: el.getAttribute("data-instances-json") }); })];
                                case 1:
                                    attrs = _a.sent();
                                    return [2 /*return*/, { meshes: jsonArrayLength(attrs.meshes), instances: jsonArrayLength(attrs.instances) }];
                            }
                        });
                    }); };
                    return [4 /*yield*/, readAttrs()];
                case 5:
                    counts = _a.sent();
                    return [4 /*yield*/, settleContentCount(function () { return __awaiter(_this, void 0, void 0, function () {
                            return __generator(this, function (_a) {
                                switch (_a.label) {
                                    case 0: return [4 /*yield*/, readAttrs()];
                                    case 1:
                                        counts = _a.sent();
                                        return [2 /*return*/, counts.meshes + counts.instances];
                                }
                            });
                        }); }, requireContent)];
                case 6:
                    _a.sent();
                    return [2 /*return*/, __assign({ hasScene: true }, counts)];
            }
        });
    });
}
/** @emoji 🕸️ `NodeGraph` stamps its live flow document onto `.semio-node-graph-host` as
 * `data-host-snapshot-json` (`NodeGraph/🟦️.tsx`'s `NodeGraphHost` render) — a real snapshot parses to a
 * JSON object carrying a `widgets[]`. That attribute was called `data-fixture-json` until commit
 * 8773331d23 ("Align plugin schemas, mutations, panels, and tests to snapshot-fixture-asset
 * terminology") renamed the scene field `fixtureJson` → `hostSnapshotJson`; the graph engine is a wasm
 * canvas with no per-node DOM, so this attribute is the only DOM-level evidence of graph content there
 * is. Measured on the 6029 serve 2026-09-16: generator's `proceduralMain` carries a 1867-byte snapshot
 * with 7 widgets and its `synapses[]` wiring. */
function nodeGraphWidgetCount(container_1) {
    return __awaiter(this, arguments, void 0, function (container, requireContent) {
        var host, empty, widgets;
        var _this = this;
        if (requireContent === void 0) { requireContent = false; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    host = container.locator(".semio-node-graph-host");
                    empty = container.locator(".semio-node-graph-empty");
                    return [4 /*yield*/, (0, test_1.expect)(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    if (!requireContent) return [3 /*break*/, 3];
                    return [4 /*yield*/, host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(function () { })];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3: return [4 /*yield*/, host.count()];
                case 4:
                    if ((_a.sent()) === 0)
                        return [2 /*return*/, { hasScene: false, widgets: 0 }];
                    return [4 /*yield*/, settleContentCount(function () { return __awaiter(_this, void 0, void 0, function () {
                            var snapshotJson;
                            var _a, _b;
                            return __generator(this, function (_c) {
                                switch (_c.label) {
                                    case 0: return [4 /*yield*/, host.getAttribute("data-host-snapshot-json")];
                                    case 1:
                                        snapshotJson = _c.sent();
                                        if (!snapshotJson)
                                            return [2 /*return*/, 0];
                                        try {
                                            return [2 /*return*/, (_b = (_a = JSON.parse(snapshotJson).widgets) === null || _a === void 0 ? void 0 : _a.length) !== null && _b !== void 0 ? _b : 0];
                                        }
                                        catch (_d) {
                                            return [2 /*return*/, 0];
                                        }
                                        return [2 /*return*/];
                                }
                            });
                        }); }, requireContent)];
                case 5:
                    widgets = _a.sent();
                    return [2 /*return*/, { hasScene: true, widgets: widgets }];
            }
        });
    });
}
/** @emoji 📊️ `Table`'s generic row primitive stamps `data-row-id` on every real data `<tr>`
 * (`framework/ui/elements/📊️Table/🟦️.tsx` lines 201/262) — counting them is a direct, DOM-level
 * "does this table have rows" check with no reliance on cell text/locale. */
function tableRowCount(container_1) {
    return __awaiter(this, arguments, void 0, function (container, requireContent) {
        var host, empty, rows;
        if (requireContent === void 0) { requireContent = false; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    host = container.locator(".semio-table-host");
                    empty = container.locator(".semio-table-empty");
                    return [4 /*yield*/, (0, test_1.expect)(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    if (!requireContent) return [3 /*break*/, 3];
                    return [4 /*yield*/, host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(function () { })];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3: return [4 /*yield*/, host.count()];
                case 4:
                    if ((_a.sent()) === 0)
                        return [2 /*return*/, { hasScene: false, rows: 0 }];
                    return [4 /*yield*/, settleContentCount(function () { return host.locator("[data-row-id]").count(); }, requireContent)];
                case 5:
                    rows = _a.sent();
                    return [2 /*return*/, { hasScene: true, rows: rows }];
            }
        });
    });
}
/** @emoji 🌳️ Two of energy's four windows (Structure, Energy simulation) have a plain framework `Tree`
 * for a body rather than a surface host — `TreeWindowKit` builds a `TreeView`, and `🌳️Tree/🟦️.tsx`
 * renders one `role="treeitem"` + `data-slot="tree-item-row"` element per row (lines ~1936/2013/2088).
 * There is no `.semio-…-host`/`.semio-…-empty` pair to look for, so "did this window resolve" is
 * answered by its `role="tree"` body attaching and "does it carry content" by the row count — both
 * production attributes the framework already emits, in the same spirit as `data-row-id` for tables. */
function treeRowCount(container_1) {
    return __awaiter(this, arguments, void 0, function (container, requireContent) {
        var body, rows;
        if (requireContent === void 0) { requireContent = false; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    body = container.locator('[role="tree"]');
                    return [4 /*yield*/, (0, test_1.expect)(body.first()).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    return [4 /*yield*/, settleContentCount(function () { return container.locator('[role="treeitem"]').count(); }, requireContent)];
                case 2:
                    rows = _a.sent();
                    return [2 /*return*/, { hasScene: true, rows: rows }];
            }
        });
    });
}
/** @emoji 🗺️ `TiledMapHost` has no content-count DOM attribute (unlike the other three surfaces), so
 * "did the map paint" has to be answered from pixels — but NOT by reading the live canvas back.
 *
 * The map is presented by the wasm/wgpu surface session, whose swap-chain is not a preserved drawing
 * buffer: `drawImage(mapCanvas, …)` + `getImageData` answers a fully transparent `0,0,0,0` image even
 * when the map is visibly drawn (measured 2026-09-16 against the 6029 serve, whose map paints
 * continents, labels and the marker — the readback was flat-transparent on both the GPU and the
 * software stack). Playwright's own element screenshot is a composited grab, so it sees what the user
 * sees; it is decoded back into an `ImageData` in the page, and only the CENTRE box is counted so the
 * window's own chrome/veil at the edges can never stand in for map content.
 *
 * Map tiles are proxied same-origin by the dev server (`framework/ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`'s
 * `createTileProxyMiddleware`, routes `/osm` + `/vt` off `.🧬semio/🗺️map`), so nothing here is
 * cross-origin-tainted and no network egress is needed. */
function tiledMapHasVisibleContent(page_1, container_1) {
    return __awaiter(this, arguments, void 0, function (page, container, requireContent) {
        var host, empty, canvas, distinctColors;
        if (requireContent === void 0) { requireContent = false; }
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    host = container.locator(".semio-tiled-map-host");
                    empty = container.locator(".semio-tiled-map-empty");
                    return [4 /*yield*/, (0, test_1.expect)(host.or(empty)).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 1:
                    _a.sent();
                    if (!requireContent) return [3 /*break*/, 3];
                    return [4 /*yield*/, host.first().waitFor({ state: "visible", timeout: SURFACE_CONTENT_TIMEOUT_MS }).catch(function () { })];
                case 2:
                    _a.sent();
                    _a.label = 3;
                case 3: return [4 /*yield*/, host.count()];
                case 4:
                    if ((_a.sent()) === 0)
                        return [2 /*return*/, { hasScene: false, painted: false, distinctColors: 0 }];
                    canvas = host.locator("canvas");
                    return [4 /*yield*/, (0, test_1.expect)(canvas).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 5:
                    _a.sent();
                    // 🐢️ Give the map session a couple of animation frames to actually paint after mount.
                    return [4 /*yield*/, canvas.evaluate(function () { return new Promise(function (resolve) { return requestAnimationFrame(function () { return requestAnimationFrame(function () { return resolve(); }); }); }); })];
                case 6:
                    // 🐢️ Give the map session a couple of animation frames to actually paint after mount.
                    _a.sent();
                    return [4 /*yield*/, settleContentCount(function () { return countCanvasColors(page, canvas); }, requireContent, 2)];
                case 7:
                    distinctColors = _a.sent();
                    return [2 /*return*/, { hasScene: true, painted: distinctColors > 1, distinctColors: distinctColors }];
            }
        });
    });
}
/** @emoji 🎨️ Distinct RGBA values in the centre 60% of one element's composited screenshot; `1` means
 * one flat colour and `0` means the grab could not be decoded. */
function countCanvasColors(page, canvas) {
    return __awaiter(this, void 0, void 0, function () {
        var shot, distinct;
        var _this = this;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, canvas.screenshot()];
                case 1:
                    shot = (_a.sent()).toString("base64");
                    return [4 /*yield*/, page.evaluate(function (base64) { return __awaiter(_this, void 0, void 0, function () {
                            var binary, bytes, i, bitmap, probe, ctx, x0, y0, w, h, data, seen, i;
                            return __generator(this, function (_a) {
                                switch (_a.label) {
                                    case 0:
                                        binary = atob(base64);
                                        bytes = new Uint8Array(binary.length);
                                        for (i = 0; i < binary.length; i += 1)
                                            bytes[i] = binary.charCodeAt(i);
                                        return [4 /*yield*/, createImageBitmap(new Blob([bytes], { type: "image/png" }))];
                                    case 1:
                                        bitmap = _a.sent();
                                        probe = document.createElement("canvas");
                                        probe.width = bitmap.width;
                                        probe.height = bitmap.height;
                                        ctx = probe.getContext("2d");
                                        if (!ctx)
                                            return [2 /*return*/, 0];
                                        ctx.drawImage(bitmap, 0, 0);
                                        x0 = Math.floor(bitmap.width * 0.2);
                                        y0 = Math.floor(bitmap.height * 0.2);
                                        w = Math.max(1, Math.floor(bitmap.width * 0.6));
                                        h = Math.max(1, Math.floor(bitmap.height * 0.6));
                                        data = ctx.getImageData(x0, y0, w, h).data;
                                        seen = new Set();
                                        for (i = 0; i < data.length; i += 4)
                                            seen.add((data[i] << 24) | (data[i + 1] << 16) | (data[i + 2] << 8) | data[i + 3]);
                                        return [2 /*return*/, seen.size];
                                }
                            });
                        }); }, shot)];
                case 2:
                    distinct = _a.sent();
                    return [2 /*return*/, distinct];
            }
        });
    });
}
//#endregion 🪟️SurfaceContent
//#region 🧪️PaneConsistency
(0, test_1.test)("DEMONSTRATOR_PANES matches the pane ids this suite covers (drift guard)", function () {
    (0, test_1.expect)(PANE_CASES.map(function (entry) { return entry.paneId; })).toEqual(brandPaneIds());
});
//#endregion 🧪️PaneConsistency
//#region 🃏️OverviewCards
/** @emoji 🃏️ Landing overview cards use window silhouettes (icon title chips, no drag handles). */
(0, test_1.test)("demonstrator overview: pane cards use window-silhouette chrome without drag handles", function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
    var pageErrors, consoleErrors, cards, _i, _c, paneId, card;
    var page = _b.page;
    return __generator(this, function (_d) {
        switch (_d.label) {
            case 0:
                test_1.test.setTimeout(TEST_TIMEOUT_MS);
                pageErrors = [];
                consoleErrors = [];
                page.on("pageerror", function (error) { return pageErrors.push(error); });
                page.on("console", function (message) {
                    if (message.type() === "error")
                        consoleErrors.push(message.text());
                });
                return [4 /*yield*/, page.goto("/", { waitUntil: "domcontentloaded" })];
            case 1:
                _d.sent();
                return [4 /*yield*/, dismissIntroduction(page, page.locator('[id="ui.introduction.skip"]'))];
            case 2:
                _d.sent();
                cards = page.locator("[data-demonstrator-pane-card]");
                return [4 /*yield*/, (0, test_1.expect)(cards).toHaveCount(brandPaneIds().length, { timeout: SHELL_READY_TIMEOUT_MS })];
            case 3:
                _d.sent();
                _i = 0, _c = brandPaneIds();
                _d.label = 4;
            case 4:
                if (!(_i < _c.length)) return [3 /*break*/, 17];
                paneId = _c[_i];
                card = page.locator("[data-demonstrator-pane-card][data-pane-id=\"".concat(paneId, "\"]"));
                return [4 /*yield*/, (0, test_1.expect)(card).toBeVisible()];
            case 5:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator("[data-window-silhouette]")).toHaveCount(1)];
            case 6:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-window-silhouette-border][data-kind="normal"]')).toHaveCount(1)];
            case 7:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="window-chrome-chip-cap"]')).toHaveClass(/ui-glass/)];
            case 8:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="window-chrome-body-surface"]')).toHaveClass(/ui-glass/)];
            case 9:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="window-chrome-body"]')).not.toHaveClass(/ui-surface/)];
            case 10:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="demonstrator-pane-card-title-chip"] svg')).toHaveCount(1)];
            case 11:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="demonstrator-pane-card-tagline"]')).toHaveText(/.+/)];
            case 12:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="introduction-body-paragraph"]')).toHaveCount(2)];
            case 13:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot="demonstrator-pane-card-open-chip"]')).toContainText("Demonstrator öffnen")];
            case 14:
                _d.sent();
                return [4 /*yield*/, (0, test_1.expect)(card.locator('[data-slot*="drag"], [data-drag-handle]')).toHaveCount(0)];
            case 15:
                _d.sent();
                _d.label = 16;
            case 16:
                _i++;
                return [3 /*break*/, 4];
            case 17:
                (0, test_1.expect)(pageErrors.map(function (error) { return error.message; }), "unexpected page errors on overview").toEqual([]);
                (0, test_1.expect)(significantConsoleErrors(consoleErrors), "unexpected console errors on overview").toEqual([]);
                return [2 /*return*/];
        }
    });
}); });
var _loop_1 = function (paneCase) {
    (0, test_1.test)("demonstrator pane \"".concat(paneCase.paneId, "\": boots via hash deep-link and renders its declared window(s)"), function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
        var pageErrors, consoleErrors, outcome, _i, _c, win, instanceIds, aliasMatches, _d, instanceIds_1, rawId, elementId, container, label, _e, hasScene, meshes, instances, _f, hasScene, widgets, _g, hasScene, rows, _h, hasScene, rows, _j, hasScene, painted, distinctColors;
        var _k;
        var page = _b.page;
        return __generator(this, function (_l) {
            switch (_l.label) {
                case 0:
                    test_1.test.setTimeout(TEST_TIMEOUT_MS);
                    pageErrors = [];
                    consoleErrors = [];
                    page.on("pageerror", function (error) { return pageErrors.push(error); });
                    page.on("console", function (message) {
                        if (message.type() === "error")
                            consoleErrors.push(message.text());
                    });
                    return [4 /*yield*/, page.goto("/#".concat(paneCase.paneId), { waitUntil: "domcontentloaded" })];
                case 1:
                    _l.sent();
                    return [4 /*yield*/, (0, test_1.expect)(page.locator("[data-shell-id=\"".concat(paneCase.paneId, "\"]"))).toHaveCount(1, { timeout: SHELL_READY_TIMEOUT_MS })];
                case 2:
                    _l.sent();
                    return [4 /*yield*/, waitForPaneShellOutcome(page, paneCase.paneId)];
                case 3:
                    outcome = _l.sent();
                    (0, test_1.expect)(outcome, "pane \"".concat(paneCase.paneId, "\": expected its shell to reach \"ready\"")).toBe("ready");
                    return [4 /*yield*/, dismissIntroductionIfPresent(page, paneCase.paneId)];
                case 4:
                    _l.sent();
                    _i = 0, _c = paneCase.windows;
                    _l.label = 5;
                case 5:
                    if (!(_i < _c.length)) return [3 /*break*/, 23];
                    win = _c[_i];
                    instanceIds = (_k = win.instanceIds) !== null && _k !== void 0 ? _k : [windowElementId(win.kindId)];
                    if (!win.instanceIds) return [3 /*break*/, 7];
                    aliasMatches = page.locator(paneElementSelector(paneCase.paneId, windowElementId(win.kindId)));
                    return [4 /*yield*/, (0, test_1.expect)(aliasMatches).toHaveCount(instanceIds.length, { timeout: SHELL_READY_TIMEOUT_MS })];
                case 6:
                    _l.sent();
                    _l.label = 7;
                case 7:
                    _d = 0, instanceIds_1 = instanceIds;
                    _l.label = 8;
                case 8:
                    if (!(_d < instanceIds_1.length)) return [3 /*break*/, 22];
                    rawId = instanceIds_1[_d];
                    elementId = win.instanceIds ? "framework.window.".concat(elementIdSegment(rawId)) : rawId;
                    container = page.locator(paneElementSelector(paneCase.paneId, elementId));
                    return [4 /*yield*/, (0, test_1.expect)(container, "pane \"".concat(paneCase.paneId, "\": window \"").concat(rawId, "\" did not attach")).toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
                case 9:
                    _l.sent();
                    label = "pane \"".concat(paneCase.paneId, "\" window \"").concat(rawId, "\"").concat(win.note ? " (".concat(win.note, ")") : "");
                    if (!(win.surface === "world3d")) return [3 /*break*/, 11];
                    return [4 /*yield*/, worldContentCount(container, win.expectContent)];
                case 10:
                    _e = _l.sent(), hasScene = _e.hasScene, meshes = _e.meshes, instances = _e.instances;
                    (0, test_1.expect)(hasScene, "".concat(label, ": world3d surface never resolved past its empty placeholder")).toBe(true);
                    if (win.expectContent)
                        (0, test_1.expect)(meshes + instances, "".concat(label, ": expected non-empty meshes/instances JSON (meshes=").concat(meshes, ", instances=").concat(instances, ")")).toBeGreaterThan(0);
                    return [3 /*break*/, 21];
                case 11:
                    if (!(win.surface === "nodeGraph")) return [3 /*break*/, 13];
                    return [4 /*yield*/, nodeGraphWidgetCount(container, win.expectContent)];
                case 12:
                    _f = _l.sent(), hasScene = _f.hasScene, widgets = _f.widgets;
                    (0, test_1.expect)(hasScene, "".concat(label, ": node-graph surface never resolved past its empty placeholder")).toBe(true);
                    if (win.expectContent)
                        (0, test_1.expect)(widgets, "".concat(label, ": expected a non-empty widgets[] in data-host-snapshot-json")).toBeGreaterThan(0);
                    return [3 /*break*/, 21];
                case 13:
                    if (!(win.surface === "table")) return [3 /*break*/, 15];
                    return [4 /*yield*/, tableRowCount(container, win.expectContent)];
                case 14:
                    _g = _l.sent(), hasScene = _g.hasScene, rows = _g.rows;
                    (0, test_1.expect)(hasScene, "".concat(label, ": table surface never resolved past its empty placeholder")).toBe(true);
                    if (win.expectContent)
                        (0, test_1.expect)(rows, "".concat(label, ": expected at least one [data-row-id] row")).toBeGreaterThan(0);
                    return [3 /*break*/, 21];
                case 15:
                    if (!(win.surface === "tree")) return [3 /*break*/, 17];
                    return [4 /*yield*/, treeRowCount(container, win.expectContent)];
                case 16:
                    _h = _l.sent(), hasScene = _h.hasScene, rows = _h.rows;
                    (0, test_1.expect)(hasScene, "".concat(label, ": tree body never attached")).toBe(true);
                    if (win.expectContent)
                        (0, test_1.expect)(rows, "".concat(label, ": expected at least one role=\"treeitem\" row")).toBeGreaterThan(0);
                    return [3 /*break*/, 21];
                case 17:
                    if (!(win.surface === "placeholder")) return [3 /*break*/, 19];
                    return [4 /*yield*/, (0, test_1.expect)(container, "".concat(label, ": expected its documented text placeholder")).toHaveText(win.placeholderPattern, { timeout: SHELL_READY_TIMEOUT_MS })];
                case 18:
                    _l.sent();
                    return [3 /*break*/, 21];
                case 19: return [4 /*yield*/, tiledMapHasVisibleContent(page, container, win.expectContent)];
                case 20:
                    _j = _l.sent(), hasScene = _j.hasScene, painted = _j.painted, distinctColors = _j.distinctColors;
                    (0, test_1.expect)(hasScene, "".concat(label, ": tiled-map surface never resolved past its empty placeholder")).toBe(true);
                    if (win.expectContent)
                        (0, test_1.expect)(painted, "".concat(label, ": expected the map canvas to paint more than one flat color (distinct colors=").concat(distinctColors, ")")).toBe(true);
                    _l.label = 21;
                case 21:
                    _d++;
                    return [3 /*break*/, 8];
                case 22:
                    _i++;
                    return [3 /*break*/, 5];
                case 23:
                    (0, test_1.expect)(pageErrors.map(function (error) { return error.message; }), "pane \"".concat(paneCase.paneId, "\": unexpected page errors")).toEqual([]);
                    (0, test_1.expect)(significantConsoleErrors(consoleErrors), "pane \"".concat(paneCase.paneId, "\": unexpected console errors")).toEqual([]);
                    return [2 /*return*/];
            }
        });
    }); });
};
//#endregion 🃏️OverviewCards
for (var _i = 0, PANE_CASES_1 = PANE_CASES; _i < PANE_CASES_1.length; _i++) {
    var paneCase = PANE_CASES_1[_i];
    _loop_1(paneCase);
}
//#region 🎯️AussuchenSelection
/** @emoji 🎯️ The other half of aussuchen's Preview window: picking a stock row must turn the
 * `built_text_node(labels.no_selection)` placeholder into a real World3d scene.
 *
 * This is the served proof of the interaction-view threading wave
 * (`📓️fix-2026-09-16-interaction-view-threading.md`): `✏️editor/🦀️.rs:1091` feeds
 * `preview::render(doc.snapshot, &selected_ids, …)` from `interaction.selection(SOURCING_ROWS_DOMAIN)`,
 * so "what the rows domain has selected" has to travel guest→host→surface for this to pass. Measured
 * on the 6029 serve 2026-09-16: before the click the window reads `Keine Auswahl` with no world host;
 * after it, `.semio-world-3d-host` carries an 804-byte `data-meshes-json`. */
(0, test_1.test)('demonstrator pane "aussuchen": selecting a stock row renders the preview scene', function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
    var _c, preview, rows, _d, hasScene, meshes, instances;
    var page = _b.page;
    return __generator(this, function (_e) {
        switch (_e.label) {
            case 0:
                test_1.test.setTimeout(TEST_TIMEOUT_MS);
                return [4 /*yield*/, page.goto("/#aussuchen", { waitUntil: "domcontentloaded" })];
            case 1:
                _e.sent();
                _c = test_1.expect;
                return [4 /*yield*/, waitForPaneShellOutcome(page, "aussuchen")];
            case 2:
                _c.apply(void 0, [_e.sent(), 'pane "aussuchen": expected its shell to reach "ready"']).toBe("ready");
                return [4 /*yield*/, dismissIntroductionIfPresent(page, "aussuchen")];
            case 3:
                _e.sent();
                preview = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-preview")));
                return [4 /*yield*/, (0, test_1.expect)(preview, "aussuchen preview starts on its documented no-selection placeholder").toHaveText(/Keine Auswahl|No selection/, { timeout: SHELL_READY_TIMEOUT_MS })];
            case 4:
                _e.sent();
                rows = page.locator(paneElementSelector("aussuchen", windowElementId("sourcing-pool"))).locator(".semio-table-host [data-row-id]");
                return [4 /*yield*/, (0, test_1.expect)(rows.first(), "aussuchen stock pool must offer a row to select").toBeVisible({ timeout: SHELL_READY_TIMEOUT_MS })];
            case 5:
                _e.sent();
                return [4 /*yield*/, rows.first().click({ timeout: 30000 })];
            case 6:
                _e.sent();
                return [4 /*yield*/, worldContentCount(preview, true)];
            case 7:
                _d = _e.sent(), hasScene = _d.hasScene, meshes = _d.meshes, instances = _d.instances;
                (0, test_1.expect)(hasScene, "aussuchen preview: selecting a row must resolve it to a world3d surface").toBe(true);
                (0, test_1.expect)(meshes + instances, "aussuchen preview: expected the selected kind's geometry (meshes=".concat(meshes, ", instances=").concat(instances, ")")).toBeGreaterThan(0);
                return [2 /*return*/];
        }
    });
}); });
//#endregion 🎯️AussuchenSelection
(0, test_1.test)("landing Förderhinweis renders non-empty funding logos", function (_a) { return __awaiter(void 0, [_a], void 0, function (_b) {
    var next, step, visibleLogoSizes, _i, visibleLogoSizes_1, logo;
    var page = _b.page;
    return __generator(this, function (_c) {
        switch (_c.label) {
            case 0:
                test_1.test.setTimeout(60000);
                return [4 /*yield*/, page.goto("/", { waitUntil: "networkidle" })];
            case 1:
                _c.sent();
                next = page.locator('[data-slot="introduction-info-box"] button').filter({ hasText: /Weiter/i });
                step = 0;
                _c.label = 2;
            case 2:
                if (!(step < 2)) return [3 /*break*/, 6];
                return [4 /*yield*/, next.click({ timeout: 15000 })];
            case 3:
                _c.sent();
                return [4 /*yield*/, page.waitForTimeout(400)];
            case 4:
                _c.sent();
                _c.label = 5;
            case 5:
                step += 1;
                return [3 /*break*/, 2];
            case 6: return [4 /*yield*/, (0, test_1.expect)(page.locator('[data-slot="introduction-info-box-title"]')).toHaveText("Förderhinweis")];
            case 7:
                _c.sent();
                return [4 /*yield*/, page.locator('[data-slot="introduction-info-box"] img').evaluateAll(function (imgs) {
                        return imgs
                            .filter(function (img) { return getComputedStyle(img).display !== "none"; })
                            .map(function (img) { return ({ width: img.clientWidth, height: img.clientHeight, naturalWidth: img.naturalWidth }); });
                    })];
            case 8:
                visibleLogoSizes = _c.sent();
                (0, test_1.expect)(visibleLogoSizes.length).toBeGreaterThanOrEqual(3);
                for (_i = 0, visibleLogoSizes_1 = visibleLogoSizes; _i < visibleLogoSizes_1.length; _i++) {
                    logo = visibleLogoSizes_1[_i];
                    (0, test_1.expect)(logo.width, "introduction funding logo should layout with width").toBeGreaterThan(0);
                    (0, test_1.expect)(logo.height, "introduction funding logo should layout with height").toBeGreaterThan(0);
                    (0, test_1.expect)(logo.naturalWidth, "introduction funding logo should decode as an image").toBeGreaterThan(0);
                }
                return [2 /*return*/];
        }
    });
}); });
