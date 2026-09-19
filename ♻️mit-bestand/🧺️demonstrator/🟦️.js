"use strict";
// #region 🧲️Header
/** @emoji 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction, eight live app panes, glass name overlay. */
// #endregion 🧲️Header
Object.defineProperty(exports, "__esModule", { value: true });
var runtime_1 = require("@semio-tech/ui-react/runtime");
var ui_react_1 = require("@semio-tech/ui-react");
var framework_1 = require("@semio-tech/framework");
var catalog_1 = require("@semio-tech/plugin-registry/catalog");
var framework_renderer_react_1 = require("@semio-tech/framework-renderer-react");
var puzzle_js_1 = require("@semio-tech/puzzle-js");
var __demonstrator_card_tsx_1 = require("./\u269B\uFE0Fdemonstrator-card.tsx");
var __footer_tsx_1 = require("./\u269B\uFE0Ffooter.tsx");
var ___brand_ts_1 = require("./\uD83E\uDEA7\uFE0Fbrand.ts");
require("./\uD83C\uDFA8\uFE0Fglobals.css");
// 🎪️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
var demonstratorStorage = (0, framework_1.createBrowserStoragePort)();
(0, ui_react_1.bootstrapElementsSurfaceChromeDocument)((0, ui_react_1.readStoredUiChromeAppearance)(demonstratorStorage));
// 🇩🇪️ The whole demonstrator is German-locked (see 🪧️brand.ts) — resolve synchronously before the
// first render so the landing page's own chrome (Skip/Back/Next/Done) never flashes English.
(0, ui_react_1.initUiLocaleSync)(___brand_ts_1.DEMONSTRATOR_LOCALE);
/** @emoji 📱️ Touch-first viewports use the vertical snap list even when wider than {@link UI_MOBILE_MEDIA_QUERY}. */
var DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY = "".concat(ui_react_1.UI_MOBILE_MEDIA_QUERY, " and (hover: none) and (pointer: coarse)");
//#region 🎪️DemonstratorGridGeometry
/** @emoji 🔢️ Columns and rows of the demonstrator preview grid; the strip spans `columns * 100vw` by `rows * 100vh`. */
var DEMONSTRATOR_GRID_COLUMNS = 4;
var DEMONSTRATOR_GRID_ROWS = 2;
function paneColumn(paneIndex) {
    return paneIndex % DEMONSTRATOR_GRID_COLUMNS;
}
function paneRow(paneIndex) {
    return Math.floor(paneIndex / DEMONSTRATOR_GRID_COLUMNS);
}
function paneIndexById(id) {
    return ___brand_ts_1.DEMONSTRATOR_PANES.findIndex(function (pane) { return pane.id === id; });
}
function paneIdFromLocationHash() {
    var raw = window.location.hash.replace(/^#/, "").trim();
    if (!raw)
        return null;
    return ___brand_ts_1.DEMONSTRATOR_PANES.some(function (pane) { return pane.id === raw; }) ? raw : null;
}
/** @emoji 🧭️ Largest scroll offset that still keeps the last column and row flush with the viewport edge. */
var DEMONSTRATOR_MAX_SCROLL = { x: (DEMONSTRATOR_GRID_COLUMNS - 1) * 100, y: (DEMONSTRATOR_GRID_ROWS - 1) * 100 };
/** @emoji 🎞 Programmatic pane pin / focus glide duration — one rAF timeline owns the transform; never pair this with a CSS `transition` on the same property. */
var DEMONSTRATOR_SCROLL_GLIDE_MS = 500;
/** @emoji 🎞 Exponential follow factor while the cursor freely pans the overview. */
var DEMONSTRATOR_SCROLL_FOLLOW_LERP = 0.12;
/** @emoji 🎞 Settle epsilon (vw/vh) for the free-pan follow loop. */
var DEMONSTRATOR_SCROLL_FOLLOW_EPSILON = 0.01;
/** @emoji 🎞 Cubic ease-in-out for focus / hover pin glides. */
function easeInOutCubic(t) {
    return t < 0.5 ? 4 * t * t * t : 1 - Math.pow((-2 * t + 2), 3) / 2;
}
/** @emoji 🧭️ Scroll offset that brings the given pane fully into the viewport. */
function scrollOffsetForPaneIndex(paneIndex) {
    return {
        x: Math.min(DEMONSTRATOR_MAX_SCROLL.x, Math.max(0, paneColumn(paneIndex) * 100)),
        y: Math.min(DEMONSTRATOR_MAX_SCROLL.y, Math.max(0, paneRow(paneIndex) * 100)),
    };
}
/** @emoji 🧭️ Linear blend of two scroll offsets. */
function lerpScrollOffset(from, to, t) {
    return { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t };
}
/** @emoji 📐 Maps one axis of a grid cell into the current viewport after scrolling (percent of that axis). */
function paneAxisBounds(cellIndex, scrollPercent) {
    var cellStart = cellIndex * 100 - scrollPercent;
    var start = Math.max(0, cellStart);
    var end = Math.min(100, cellStart + 100);
    return { start: start, end: end, visible: end > start };
}
/** @emoji 👁 Visible on-screen bounds of a grid pane — the region that stays untinted while its card is hovered. */
function demonstratorPaneRevealRect(paneIndex, scrollOffset) {
    var horizontal = paneAxisBounds(paneColumn(paneIndex), scrollOffset.x);
    var vertical = paneAxisBounds(paneRow(paneIndex), scrollOffset.y);
    if (!horizontal.visible || !vertical.visible)
        return { top: 0, left: 0, width: 0, height: 0 };
    var vw = window.innerWidth;
    var vh = window.innerHeight;
    var left = (horizontal.start / 100) * vw;
    var right = (horizontal.end / 100) * vw;
    var top = (vertical.start / 100) * vh;
    var bottom = (vertical.end / 100) * vh;
    return { top: top, left: left, width: Math.max(0, right - left), height: Math.max(0, bottom - top) };
}
/** @emoji 🪟️ Full-viewport veil pieces; optional rectangular cutout leaves the hovered app pane untinted. */
function demonstratorTintSegmentsPx(revealRect) {
    var vw = window.innerWidth;
    var vh = window.innerHeight;
    if (!revealRect)
        return [{ top: 0, left: 0, width: vw, height: vh }];
    var holeLeft = Math.max(0, revealRect.left);
    var holeTop = Math.max(0, revealRect.top);
    var holeRight = Math.min(vw, revealRect.left + revealRect.width);
    var holeBottom = Math.min(vh, revealRect.top + revealRect.height);
    if (holeRight <= holeLeft || holeBottom <= holeTop)
        return [{ top: 0, left: 0, width: vw, height: vh }];
    var segments = [];
    if (holeTop > 0)
        segments.push({ top: 0, left: 0, width: vw, height: holeTop });
    if (holeBottom < vh)
        segments.push({ top: holeBottom, left: 0, width: vw, height: vh - holeBottom });
    if (holeLeft > 0)
        segments.push({ top: holeTop, left: 0, width: holeLeft, height: holeBottom - holeTop });
    if (holeRight < vw)
        segments.push({ top: holeTop, left: holeRight, width: vw - holeRight, height: holeBottom - holeTop });
    return segments;
}
//#endregion 🎪️DemonstratorGridGeometry
//#region 📱️DemonstratorMobileList
/** @emoji 🌫️ Touch overview keeps a full veil over each live pane — settled sections stay blurred so background apps never read clearly through the card. */
var DEMONSTRATOR_MOBILE_OVERVIEW_VEIL_OPACITY = 1;
//#endregion 📱️DemonstratorMobileList
//#region 🎪️DemonstratorPaneBoot
/** @emoji ⏱️ Keeps each background shell's 30-second plugin-load budget isolated from the next boot. */
var DEMONSTRATOR_PANE_BOOT_INTERVAL_MS = 35000;
/** @emoji 🐢️ Boots panes one at a time (hash-target pane first, if any) instead of all eight simultaneously —
 * eight live WASM plugin boots at once would make the very first paint of the page janky. `promote` lets a
 * hover/focus jump a not-yet-booted pane to the front, since the user is about to look at it right now. */
function useSequentialPaneBoot(initialFocusId, options) {
    var _a;
    var _b = (0, runtime_1.useUiState)(function () { return new Set(initialFocusId ? [initialFocusId] : []); }), bootedIds = _b[0], setBootedIds = _b[1];
    var queueRef = (0, runtime_1.useUiRef)(___brand_ts_1.DEMONSTRATOR_PANES.map(function (pane) { return pane.id; }).filter(function (id) { return id !== initialFocusId; }));
    var cancelRef = (0, runtime_1.useUiRef)(null);
    var skipIdleQueue = (_a = options === null || options === void 0 ? void 0 : options.skipIdleQueue) !== null && _a !== void 0 ? _a : false;
    var boot = (0, runtime_1.useUiCallback)(function (id) {
        setBootedIds(function (prev) { return (prev.has(id) ? prev : new Set(prev).add(id)); });
        queueRef.current = queueRef.current.filter(function (queuedId) { return queuedId !== id; });
    }, []);
    (0, runtime_1.useUiEffect)(function () {
        if (skipIdleQueue)
            return;
        var bootNext = function () {
            var nextId = queueRef.current[0];
            if (!nextId)
                return;
            boot(nextId);
            cancelRef.current = (0, ___brand_ts_1.scheduleDemonstratorIdle)(bootNext, DEMONSTRATOR_PANE_BOOT_INTERVAL_MS, window);
        };
        cancelRef.current = (0, ___brand_ts_1.scheduleDemonstratorIdle)(bootNext, 1500, window);
        return function () { var _a; return (_a = cancelRef.current) === null || _a === void 0 ? void 0 : _a.call(cancelRef); };
    }, [boot, skipIdleQueue]);
    var promote = (0, runtime_1.useUiCallback)(function (id) { return boot(id); }, [boot]);
    return { bootedIds: bootedIds, promote: promote };
}
//#endregion 🎪️DemonstratorPaneBoot
//#region 🎪️DemonstratorSuspension
/** @emoji 🎪️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: a booted pane that's fully offscreen or the
 * tab is backgrounded releases its live shell (plugin worker, WASM instances, WebGL contexts — see
 * the framework's teardown-on-unmount path) and shows a static poster instead, revived instantly on
 * hover/focus. Only PRISTINE panes (never interacted with) are ever suspended: the framework's
 * document round-trip is real now (`readAppDocumentPack`/`loadAppDocumentPack`,
 * `PluginRuntime/🟦️component.tsx`), but this demonstrator has not been wired to snapshot/restore an
 * interacted pane's live document across a suspend/resume cycle, so suspending one today would still
 * silently discard the user's work — that wiring, not a channel gap, is what's left.
 * This covers exactly the idle-tab case the demonstrator is mostly used for (an unattended
 * kiosk/booth screen) without any feature loss for interactive use — see `DemonstratorPane`'s
 * `onPointerDownCapture`/`onKeyDownCapture`, which permanently exempt a pane the moment it's touched. */
var DEMONSTRATOR_SUSPENSION_POLICY = {
    /** Booted pane fully offscreen (another pane is focused) — safe to release quickly. */
    offscreenSuspendDelayMs: 30000,
    /** Booted pane sitting idle on the overview grid (nothing focused, no recent input). */
    overviewIdleSuspendMs: 5 * 60000,
    /** Tab backgrounded — release aggressively regardless of the other two timers. */
    hiddenTabSuspendMs: 60000,
    /** How often the suspension sweep re-evaluates every booted pane. */
    sweepIntervalMs: 5000,
};
/** @emoji 🖼️ Composites every canvas inside a pane's container into one offscreen 2D canvas and
 * returns it as a data URL — must run synchronously (not after a `requestAnimationFrame`, by which
 * point a `preserveDrawingBuffer: false` WebGL backbuffer may already be cleared). Returns `null`
 * when the pane has no canvases yet or every one samples blank; callers fall back to the existing
 * "wird vorbereitet" placeholder visual in that case — no new failure mode. */
function capturePanePoster(container) {
    var canvases = container.querySelectorAll("canvas");
    if (canvases.length === 0)
        return null;
    var containerRect = container.getBoundingClientRect();
    if (containerRect.width <= 0 || containerRect.height <= 0)
        return null;
    var poster = document.createElement("canvas");
    poster.width = Math.round(containerRect.width);
    poster.height = Math.round(containerRect.height);
    var ctx = poster.getContext("2d");
    if (!ctx)
        return null;
    var drewSomething = false;
    canvases.forEach(function (canvas) {
        if (canvas.width === 0 || canvas.height === 0)
            return;
        var rect = canvas.getBoundingClientRect();
        try {
            ctx.drawImage(canvas, rect.left - containerRect.left, rect.top - containerRect.top, rect.width, rect.height);
            drewSomething = true;
        }
        catch (_a) {
            /* tainted canvas or a lost GPU context — skip it; other canvases (or the placeholder fallback) still work */
        }
    });
    if (!drewSomething)
        return null;
    try {
        return poster.toDataURL("image/png");
    }
    catch (_a) {
        return null;
    }
}
/** @emoji 🎪️ Tracks which booted panes are pristine (never interacted with), suspended (poster shown,
 * live shell released), and their captured posters — plus the sweep that suspends eligible panes on
 * the {@link DEMONSTRATOR_SUSPENSION_POLICY} schedule. `focusedId` and (while nothing is focused) the
 * most-recently-focused pane are always exempt, matching the policy's `keepLiveCount: 1`. */
function usePaneSuspension(bootedIds, focusedId, initialFocusId) {
    var _a, _b;
    var _c = (0, runtime_1.useUiState)(new Set()), dirtyIds = _c[0], setDirtyIds = _c[1];
    var _d = (0, runtime_1.useUiState)(new Set()), suspendedIds = _d[0], setSuspendedIds = _d[1];
    var _e = (0, runtime_1.useUiState)(new Map()), postersById = _e[0], setPostersById = _e[1];
    var containersRef = (0, runtime_1.useUiRef)(new Map());
    var unfocusedSinceRef = (0, runtime_1.useUiRef)(new Map());
    var prevFocusedIdRef = (0, runtime_1.useUiRef)(focusedId);
    var mostRecentFocusedIdRef = (0, runtime_1.useUiRef)((_b = initialFocusId !== null && initialFocusId !== void 0 ? initialFocusId : (_a = ___brand_ts_1.DEMONSTRATOR_PANES[0]) === null || _a === void 0 ? void 0 : _a.id) !== null && _b !== void 0 ? _b : null);
    var markDirty = (0, runtime_1.useUiCallback)(function (id) {
        setDirtyIds(function (prev) { return (prev.has(id) ? prev : new Set(prev).add(id)); });
    }, []);
    var registerContainer = (0, runtime_1.useUiCallback)(function (id, el) {
        if (el)
            containersRef.current.set(id, el);
        else
            containersRef.current.delete(id);
    }, []);
    var suspendPane = (0, runtime_1.useUiCallback)(function (id) {
        var container = containersRef.current.get(id);
        if (container) {
            var poster_1 = capturePanePoster(container);
            if (poster_1)
                setPostersById(function (prev) { return new Map(prev).set(id, poster_1); });
        }
        setSuspendedIds(function (prev) { return (prev.has(id) ? prev : new Set(prev).add(id)); });
    }, []);
    var resumePane = (0, runtime_1.useUiCallback)(function (id) {
        unfocusedSinceRef.current.delete(id);
        setSuspendedIds(function (prev) {
            if (!prev.has(id))
                return prev;
            var next = new Set(prev);
            next.delete(id);
            return next;
        });
    }, []);
    // Records when a pane most recently stopped being focused (absence from the map = currently focused).
    (0, runtime_1.useUiEffect)(function () {
        var prev = prevFocusedIdRef.current;
        if (prev && prev !== focusedId)
            unfocusedSinceRef.current.set(prev, Date.now());
        if (focusedId) {
            unfocusedSinceRef.current.delete(focusedId);
            mostRecentFocusedIdRef.current = focusedId;
        }
        prevFocusedIdRef.current = focusedId;
    }, [focusedId]);
    // A pane booted via the warm-boot queue (never focused) still needs an "unfocused since" baseline.
    (0, runtime_1.useUiEffect)(function () {
        for (var _i = 0, bootedIds_1 = bootedIds; _i < bootedIds_1.length; _i++) {
            var id = bootedIds_1[_i];
            if (id !== focusedId && !unfocusedSinceRef.current.has(id))
                unfocusedSinceRef.current.set(id, Date.now());
        }
    }, [bootedIds, focusedId]);
    (0, runtime_1.useUiEffect)(function () {
        var sweep = function () {
            var now = Date.now();
            var hidden = document.hidden;
            for (var _i = 0, bootedIds_2 = bootedIds; _i < bootedIds_2.length; _i++) {
                var id = bootedIds_2[_i];
                if (id === focusedId)
                    continue;
                if (!focusedId && id === mostRecentFocusedIdRef.current)
                    continue;
                if (dirtyIds.has(id) || suspendedIds.has(id))
                    continue;
                var since = unfocusedSinceRef.current.get(id);
                if (since == null)
                    continue;
                var threshold = hidden ? DEMONSTRATOR_SUSPENSION_POLICY.hiddenTabSuspendMs : focusedId ? DEMONSTRATOR_SUSPENSION_POLICY.offscreenSuspendDelayMs : DEMONSTRATOR_SUSPENSION_POLICY.overviewIdleSuspendMs;
                if (now - since >= threshold)
                    suspendPane(id);
            }
        };
        var interval = window.setInterval(sweep, DEMONSTRATOR_SUSPENSION_POLICY.sweepIntervalMs);
        return function () { return window.clearInterval(interval); };
    }, [bootedIds, focusedId, dirtyIds, suspendedIds, suspendPane]);
    return { dirtyIds: dirtyIds, suspendedIds: suspendedIds, postersById: postersById, markDirty: markDirty, registerContainer: registerContainer, resumePane: resumePane };
}
var PaneErrorBoundary = (0, runtime_1.createUiErrorBoundary)({
    initialState: { error: null },
    deriveState: function (error) { return ({ error: error }); },
    didCatch: function (props, error) { return console.error("Demonstrator pane \"".concat(props.paneLabel, "\" crashed"), error); },
    render: function (props, state) {
        if (state.error) {
            return (<div className="flex h-full w-full items-center justify-center bg-background p-double text-center text-sm text-muted-foreground">
          {props.paneLabel} konnte nicht geladen werden.
        </div>);
        }
        return props.children;
    },
});
//#endregion 🎪️PaneErrorBoundary
//#region 🎪️DemonstratorPane
/** @emoji 🎪️ One grid cell: either the brand-logo placeholder (not booted yet) or the live shell, wrapped
 * `inert` while not focused so it never steals pointer/keyboard/focus from whichever pane IS focused (or
 * from the overview's own hover cards) — the shell still renders and animates underneath, just inertly. */
function DemonstratorPane(_a) {
    var pane = _a.pane, booted = _a.booted, focused = _a.focused, suspended = _a.suspended, posterDataUrl = _a.posterDataUrl, onDirty = _a.onDirty, onContainerElement = _a.onContainerElement;
    var bootVariants = (0, ___brand_ts_1.demonstratorPaneBootVariants)(pane.variant);
    var runtimeBoot = (0, runtime_1.useUiMemo)(function () { return (0, framework_1.resolvePlaygroundBoot)(catalog_1.PLUGIN_CATALOG, bootVariants.runtime); }, [bootVariants.runtime]);
    var manifestBoot = (0, runtime_1.useUiMemo)(function () { return (0, framework_1.resolvePlaygroundBoot)(catalog_1.PLUGIN_CATALOG, bootVariants.manifest); }, [bootVariants.manifest]);
    var locks = (0, runtime_1.useUiMemo)(function () { return (0, framework_renderer_react_1.resolveShellLocks)(pane.brand.locks); }, [pane.brand]);
    var defaults = (0, runtime_1.useUiMemo)(function () { return (0, framework_renderer_react_1.resolveShellDefaults)(pane.brand, undefined); }, [pane.brand]);
    var live = booted && !suspended;
    return (<div ref={function (el) { return onContainerElement(pane.id, el); }} className="relative h-full w-full overflow-hidden bg-background" inert={!focused} 
    // 🎪️ Only a focused (non-inert) pane can ever actually receive these — capture-phase so a click
    // deep inside the shell (a button, a canvas) still marks the pane dirty before anything inside
    // it can stop propagation. See `usePaneSuspension`'s docstring for why this permanently exempts
    // the pane from suspension.
    onPointerDownCapture={onDirty} onKeyDownCapture={onDirty}>
      {live ? (<PaneErrorBoundary paneLabel={pane.label}>
          <framework_renderer_react_1.FrameworkOsShell pluginFilter={bootVariants.runtime} plugins={runtimeBoot.plugins} surfaceSessionFactories={puzzle_js_1.PUZZLE_BOARD_SESSION_FACTORIES} appId={manifestBoot.defaultAppId} locks={locks} defaults={defaults} brand={pane.brand} shellId={pane.id} storageNamespace={pane.id} suppressAutoIntroduction={!focused}/>
        </PaneErrorBoundary>) : booted && suspended && posterDataUrl ? (<img src={posterDataUrl} alt="" className="h-full w-full object-cover" aria-hidden/>) : (<div className={(0, ui_react_1.cn)("flex h-full w-full flex-col items-center justify-center gap-double bg-background", ui_react_1.loadingBorderClass)} role="status" aria-busy="true">
          <div className="size-huge text-foreground opacity-40 [&_svg]:h-full [&_svg]:w-full" dangerouslySetInnerHTML={{ __html: ___brand_ts_1.ENTWERFEN_MIT_BESTAND_LOGO_SVG }} aria-hidden/>
          <div className="h-full min-h-0 w-full max-w-4xl flex-1 p-double">
            <ui_react_1.CanvasSkeleton label={"".concat(pane.label, " wird vorbereitet")}/>
          </div>
        </div>)}
    </div>);
}
//#endregion 🎪️DemonstratorPane
//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
    var viewportMobile = (0, ui_react_1.useMediaQuery)(ui_react_1.UI_MOBILE_MEDIA_QUERY);
    var touchListMode = (0, ui_react_1.useMediaQuery)(DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY);
    var surfaceChrome = (0, runtime_1.useUiMemo)(function () { return ({
        appearance: (0, ui_react_1.readStoredUiChromeAppearance)(demonstratorStorage),
        device: (viewportMobile ? "mobile" : (0, ui_react_1.readStoredUiChromeLayout)(demonstratorStorage) === "tablet" ? "tablet" : "desktop"),
        driver: (0, ui_react_1.readStoredUiDriver)(demonstratorStorage),
    }); }, [viewportMobile]);
    (0, ui_react_1.useElementsSurfaceChrome)(surfaceChrome);
    var initialFocusId = (0, runtime_1.useUiMemo)(function () { return paneIdFromLocationHash(); }, []);
    var _a = (0, runtime_1.useUiState)(0), introductionStep = _a[0], setIntroductionStep = _a[1];
    var _b = (0, runtime_1.useUiState)(!initialFocusId), showIntroduction = _b[0], setShowIntroduction = _b[1];
    var _c = (0, runtime_1.useUiState)(initialFocusId), focusedId = _c[0], setFocusedId = _c[1];
    var _d = useSequentialPaneBoot(initialFocusId, { skipIdleQueue: touchListMode || focusedId != null }), bootedIds = _d.bootedIds, promote = _d.promote;
    var _e = usePaneSuspension(bootedIds, focusedId, initialFocusId), suspendedIds = _e.suspendedIds, postersById = _e.postersById, markDirty = _e.markDirty, registerContainer = _e.registerContainer, resumePane = _e.resumePane;
    var promoteAndResume = (0, runtime_1.useUiCallback)(function (id) {
        promote(id);
        resumePane(id);
    }, [promote, resumePane]);
    var _f = (0, runtime_1.useUiState)(null), hoveredPaneId = _f[0], setHoveredPaneId = _f[1];
    var _g = (0, runtime_1.useUiState)(null), revealRect = _g[0], setRevealRect = _g[1];
    var hoveredPaneIdRef = (0, runtime_1.useUiRef)(null);
    var scrollTargetRef = (0, runtime_1.useUiRef)(initialFocusId ? scrollOffsetForPaneIndex(paneIndexById(initialFocusId)) : { x: 0, y: 0 });
    var scrollCurrentRef = (0, runtime_1.useUiRef)(scrollTargetRef.current);
    var scrollDriveRef = (0, runtime_1.useUiRef)({ mode: "follow" });
    /** @emoji 🎞 Bumped on every drive change so a stale follow `setScrollOffset` cannot paint after a glide has taken ownership. */
    var scrollEpochRef = (0, runtime_1.useUiRef)(0);
    var _h = (0, runtime_1.useUiState)(scrollTargetRef.current), scrollOffset = _h[0], setScrollOffset = _h[1];
    var listScrollRef = (0, runtime_1.useUiRef)(null);
    var listProgressRafRef = (0, runtime_1.useUiRef)(null);
    var _j = (0, runtime_1.useUiState)(0), listProgress = _j[0], setListProgress = _j[1];
    var _k = (0, runtime_1.useUiState)(Boolean(initialFocusId)), listScrollLocked = _k[0], setListScrollLocked = _k[1];
    // 📽 One rAF loop drives the grid transform; it stops once settled so an idle tab isn't animating forever.
    // `ensureScrollLoopRef` restarts it whenever follow or glide needs another frame.
    var scrollLoopRunningRef = (0, runtime_1.useUiRef)(false);
    var ensureScrollLoopRef = (0, runtime_1.useUiRef)(function () { });
    var commitScrollOffset = (0, runtime_1.useUiCallback)(function (epoch, next) {
        scrollCurrentRef.current = next;
        setScrollOffset(function (prev) {
            if (scrollEpochRef.current !== epoch)
                return prev;
            return prev.x === next.x && prev.y === next.y ? prev : next;
        });
    }, []);
    var applyPaneScroll = (0, runtime_1.useUiCallback)(function (paneIndex) {
        var offset = scrollOffsetForPaneIndex(paneIndex);
        var from = scrollCurrentRef.current;
        scrollTargetRef.current = offset;
        scrollEpochRef.current += 1;
        var epoch = scrollEpochRef.current;
        if (Math.abs(offset.x - from.x) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON && Math.abs(offset.y - from.y) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON) {
            scrollDriveRef.current = { mode: "follow" };
            commitScrollOffset(epoch, offset);
            return;
        }
        scrollDriveRef.current = {
            mode: "glide",
            from: { x: from.x, y: from.y },
            to: offset,
            startedAt: performance.now(),
            durationMs: DEMONSTRATOR_SCROLL_GLIDE_MS,
        };
        // Pin the origin immediately so any already-queued follow paint is superseded before the first glide frame.
        commitScrollOffset(epoch, from);
        ensureScrollLoopRef.current();
    }, [commitScrollOffset]);
    var scrollListToPaneIndex = (0, runtime_1.useUiCallback)(function (paneIndex) {
        var el = listScrollRef.current;
        if (!el)
            return;
        var height = el.clientHeight;
        if (height <= 0)
            return;
        el.scrollTop = paneIndex * height;
        setListProgress(paneIndex);
    }, []);
    var focusPane = (0, runtime_1.useUiCallback)(function (id) {
        promoteAndResume(id);
        setFocusedId(id);
        setShowIntroduction(false);
        hoveredPaneIdRef.current = null;
        setHoveredPaneId(null);
        setRevealRect(null);
        var paneIndex = paneIndexById(id);
        if (touchListMode) {
            scrollListToPaneIndex(paneIndex);
            setListScrollLocked(true);
        }
        else {
            applyPaneScroll(paneIndex);
        }
        window.history.replaceState(null, "", "#".concat(id));
    }, [promoteAndResume, applyPaneScroll, touchListMode, scrollListToPaneIndex]);
    var returnToOverview = (0, runtime_1.useUiCallback)(function () {
        var previousFocus = focusedId;
        setFocusedId(null);
        if (touchListMode) {
            setListScrollLocked(false);
            if (previousFocus) {
                var paneIndex_1 = paneIndexById(previousFocus);
                if (paneIndex_1 >= 0)
                    requestAnimationFrame(function () { return scrollListToPaneIndex(paneIndex_1); });
            }
        }
        window.history.replaceState(null, "", window.location.pathname + window.location.search);
    }, [touchListMode, focusedId, scrollListToPaneIndex]);
    (0, runtime_1.useUiEffect)(function () {
        var onHashChange = function () {
            var paneId = paneIdFromLocationHash();
            if (paneId)
                focusPane(paneId);
            else
                returnToOverview();
        };
        window.addEventListener("hashchange", onHashChange);
        return function () { return window.removeEventListener("hashchange", onHashChange); };
    }, [focusPane, returnToOverview]);
    (0, runtime_1.useUiEffect)(function () {
        if (!focusedId)
            return;
        var onKeyDown = function (event) {
            if (event.key === "Escape")
                returnToOverview();
        };
        window.addEventListener("keydown", onKeyDown);
        return function () { return window.removeEventListener("keydown", onKeyDown); };
    }, [focusedId, returnToOverview]);
    (0, runtime_1.useUiEffect)(function () {
        if (!touchListMode || !initialFocusId)
            return;
        var paneIndex = paneIndexById(initialFocusId);
        if (paneIndex < 0)
            return;
        requestAnimationFrame(function () { return scrollListToPaneIndex(paneIndex); });
    }, [touchListMode, initialFocusId, scrollListToPaneIndex]);
    var handleListScroll = (0, runtime_1.useUiCallback)(function () {
        var el = listScrollRef.current;
        if (!el || listScrollLocked)
            return;
        if (listProgressRafRef.current != null)
            return;
        listProgressRafRef.current = requestAnimationFrame(function () {
            listProgressRafRef.current = null;
            var height = el.clientHeight;
            if (height > 0)
                setListProgress(el.scrollTop / height);
        });
    }, [listScrollLocked]);
    (0, runtime_1.useUiEffect)(function () {
        if (!touchListMode || focusedId)
            return;
        var current = Math.round(listProgress);
        var pane = ___brand_ts_1.DEMONSTRATOR_PANES[current];
        if (pane)
            promoteAndResume(pane.id);
        var next = ___brand_ts_1.DEMONSTRATOR_PANES[current + 1];
        if (next)
            promoteAndResume(next.id);
    }, [touchListMode, listProgress, focusedId, promoteAndResume]);
    var refreshRevealRect = (0, runtime_1.useUiCallback)(function (paneId, offset) {
        if (!paneId) {
            setRevealRect(null);
            return;
        }
        var paneIndex = paneIndexById(paneId);
        if (paneIndex < 0) {
            setRevealRect(null);
            return;
        }
        setRevealRect(demonstratorPaneRevealRect(paneIndex, offset));
    }, []);
    var tintSegments = (0, runtime_1.useUiMemo)(function () { return demonstratorTintSegmentsPx(revealRect); }, [revealRect]);
    (0, runtime_1.useUiEffect)(function () {
        if (touchListMode)
            return;
        var onResize = function () {
            if (hoveredPaneId)
                refreshRevealRect(hoveredPaneId, scrollCurrentRef.current);
            else
                setRevealRect(null);
        };
        window.addEventListener("resize", onResize);
        return function () { return window.removeEventListener("resize", onResize); };
    }, [touchListMode, hoveredPaneId, refreshRevealRect]);
    // 🖱️ Mouse-follow panning only makes sense in overview — a focused pane owns the mouse.
    (0, runtime_1.useUiEffect)(function () {
        if (touchListMode || focusedId)
            return;
        var onMove = function (event) {
            if (hoveredPaneIdRef.current)
                return;
            if (scrollDriveRef.current.mode !== "follow")
                scrollEpochRef.current += 1;
            scrollDriveRef.current = { mode: "follow" };
            scrollTargetRef.current = {
                x: (event.clientX / window.innerWidth) * DEMONSTRATOR_MAX_SCROLL.x,
                y: (event.clientY / window.innerHeight) * DEMONSTRATOR_MAX_SCROLL.y,
            };
            ensureScrollLoopRef.current();
        };
        window.addEventListener("mousemove", onMove, { passive: true });
        return function () { return window.removeEventListener("mousemove", onMove); };
    }, [touchListMode, focusedId]);
    (0, runtime_1.useUiEffect)(function () {
        if (touchListMode)
            return;
        var frame = 0;
        var tick = function (_frameTime) {
            // Use `performance.now()` (same clock as glide `startedAt`) — the rAF timestamp can lag slightly
            // behind and yield a negative ease `t`, which lerps backward for one frame (visible flicker).
            var now = performance.now();
            var epoch = scrollEpochRef.current;
            var drive = scrollDriveRef.current;
            if (drive.mode === "glide") {
                var t = Math.min(1, Math.max(0, (now - drive.startedAt) / drive.durationMs));
                var next_1 = lerpScrollOffset(drive.from, drive.to, easeInOutCubic(t));
                commitScrollOffset(epoch, next_1);
                if (hoveredPaneIdRef.current)
                    refreshRevealRect(hoveredPaneIdRef.current, next_1);
                if (t >= 1) {
                    scrollTargetRef.current = drive.to;
                    scrollDriveRef.current = { mode: "follow" };
                    scrollLoopRunningRef.current = false;
                    return;
                }
                frame = requestAnimationFrame(tick);
                return;
            }
            var current = scrollCurrentRef.current;
            var target = scrollTargetRef.current;
            if (Math.abs(target.x - current.x) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON && Math.abs(target.y - current.y) < DEMONSTRATOR_SCROLL_FOLLOW_EPSILON) {
                if (current.x !== target.x || current.y !== target.y) {
                    commitScrollOffset(epoch, target);
                    if (hoveredPaneIdRef.current)
                        refreshRevealRect(hoveredPaneIdRef.current, target);
                }
                scrollLoopRunningRef.current = false;
                return;
            }
            var next = lerpScrollOffset(current, target, DEMONSTRATOR_SCROLL_FOLLOW_LERP);
            // A focus/hover glide may have started after this follow sample was computed — never let the stale
            // follow frame yank the transform backward for one paint (that reads as a flicker).
            if (scrollDriveRef.current.mode === "glide" || scrollEpochRef.current !== epoch) {
                frame = requestAnimationFrame(tick);
                return;
            }
            commitScrollOffset(epoch, next);
            if (hoveredPaneIdRef.current)
                refreshRevealRect(hoveredPaneIdRef.current, next);
            frame = requestAnimationFrame(tick);
        };
        ensureScrollLoopRef.current = function () {
            if (scrollLoopRunningRef.current)
                return;
            scrollLoopRunningRef.current = true;
            frame = requestAnimationFrame(tick);
        };
        ensureScrollLoopRef.current();
        return function () {
            scrollLoopRunningRef.current = false;
            cancelAnimationFrame(frame);
        };
    }, [touchListMode, refreshRevealRect, commitScrollOffset]);
    var dismissIntroduction = (0, runtime_1.useUiCallback)(function (_completed) {
        setShowIntroduction(false);
    }, []);
    var overviewChrome = (<>
      {!hoveredPaneId && (<div className="pointer-events-none absolute inset-x-0 bottom-0 z-40 flex items-center justify-between gap-tiny px-double py-single">
          <div className="pointer-events-auto">{(0, __footer_tsx_1.aProjectOfLuhUdkFooterItem)("landingProjectOf", "de", false).content}</div>
          <div className="pointer-events-auto">{(0, __footer_tsx_1.fundedByZukunftBauFooterItem)("landingFundedBy", "de", false).content}</div>
        </div>)}

      {showIntroduction && (<ui_react_1.UIIntroduction introduction={___brand_ts_1.ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION} stepIndex={introductionStep} completedInteractionIndices={[]} onStepIndexChange={setIntroductionStep} onDismiss={dismissIntroduction}/>)}

      {!hoveredPaneId && (<ui_react_1.Navbar items={[
                {
                    key: "logoAndTitle",
                    centered: true,
                    content: (<div className="flex min-w-0 shrink-0 items-center gap-single">
                  <ui_react_1.ShellBrandLogo svg={___brand_ts_1.ENTWERFEN_MIT_BESTAND_LOGO_SVG} className="size-workbench shrink-0"/>
                  <span data-slot="app-name" className="px-single text-sm font-semibold text-foreground">
                    Entwerfen mit Bestand
                  </span>
                </div>),
                },
            ]} showFullscreenToggle={false} className="pointer-events-none absolute inset-x-0 top-0 z-40 bg-transparent"/>)}
    </>);
    var overviewReturnButton = focusedId ? (<button type="button" onClick={returnToOverview} className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single rounded-md border border-border-normal px-single py-half text-sm font-medium text-foreground shadow-md outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring">
      <ui_react_1.Icon icon="layout-grid" size="small"/>
      Übersicht
    </button>) : null;
    if (touchListMode) {
        return (<div className="relative h-full w-full overflow-hidden bg-background text-foreground">
        <div ref={listScrollRef} data-demonstrator-list-scroll="" onScroll={handleListScroll} className={(0, ui_react_1.cn)("flex w-full flex-col overscroll-y-contain", listScrollLocked ? "overflow-hidden" : "snap-y snap-mandatory overflow-y-auto")} style={{ height: "100dvh" }}>
          {___brand_ts_1.DEMONSTRATOR_PANES.map(function (pane, paneIndex) {
                var _a;
                var isFocused = focusedId === pane.id;
                var showOverviewLayer = !focusedId;
                var veilOpacity = showOverviewLayer ? DEMONSTRATOR_MOBILE_OVERVIEW_VEIL_OPACITY : 0;
                return (<section key={pane.id} className="relative w-full shrink-0 snap-start overflow-hidden" style={{ height: "100dvh", minHeight: "100dvh" }}>
                <DemonstratorPane pane={pane} booted={bootedIds.has(pane.id)} focused={isFocused} suspended={suspendedIds.has(pane.id)} posterDataUrl={(_a = postersById.get(pane.id)) !== null && _a !== void 0 ? _a : null} onDirty={function () { return markDirty(pane.id); }} onContainerElement={registerContainer}/>
                {showOverviewLayer && (<>
                    <div className="pointer-events-none absolute inset-0 z-30">
                      <div className="ui-veil absolute inset-0" style={{ opacity: veilOpacity }}/>
                    </div>
                    <div className="pointer-events-none absolute inset-0 z-[31] flex items-center justify-center px-double pb-[5.5rem]">
                      <__demonstrator_card_tsx_1.DemonstratorCard pane={pane} onClick={function () { return focusPane(pane.id); }}/>
                    </div>
                  </>)}
              </section>);
            })}
        </div>

        {!focusedId && overviewChrome}

        {overviewReturnButton}
      </div>);
    }
    return (<div className="relative h-full w-full overflow-hidden bg-background text-foreground">
      <div className="grid" style={{
            gridTemplateColumns: "repeat(".concat(DEMONSTRATOR_GRID_COLUMNS, ", 100vw)"),
            gridTemplateRows: "repeat(".concat(DEMONSTRATOR_GRID_ROWS, ", 100vh)"),
            width: "".concat(DEMONSTRATOR_GRID_COLUMNS * 100, "vw"),
            height: "".concat(DEMONSTRATOR_GRID_ROWS * 100, "vh"),
            transform: "translate(-".concat(scrollOffset.x, "vw, -").concat(scrollOffset.y, "vh)"),
        }}>
        {___brand_ts_1.DEMONSTRATOR_PANES.map(function (pane) {
            var _a;
            return (<DemonstratorPane key={pane.id} pane={pane} booted={bootedIds.has(pane.id)} focused={focusedId === pane.id} suspended={suspendedIds.has(pane.id)} posterDataUrl={(_a = postersById.get(pane.id)) !== null && _a !== void 0 ? _a : null} onDirty={function () { return markDirty(pane.id); }} onContainerElement={registerContainer}/>);
        })}
      </div>

      {!focusedId && (<>
          <div className="pointer-events-none absolute inset-0 z-30">
            {tintSegments.map(function (segment, index) { return (<div key={"tint-".concat(index, "-").concat(segment.top, "-").concat(segment.left)} className="ui-veil absolute" style={{ top: segment.top, left: segment.left, width: segment.width, height: segment.height }}/>); })}
          </div>

          <div className="pointer-events-none absolute inset-0 z-[31] grid items-center" style={{ gridTemplateColumns: "repeat(".concat(DEMONSTRATOR_GRID_COLUMNS, ", minmax(0, 1fr))"), gridTemplateRows: "repeat(".concat(DEMONSTRATOR_GRID_ROWS, ", minmax(0, 1fr))") }}>
            {___brand_ts_1.DEMONSTRATOR_PANES.map(function (pane, paneIndex) {
                var lifted = hoveredPaneId === pane.id;
                return (<div key={pane.id} className="flex justify-center px-double">
                  <__demonstrator_card_tsx_1.DemonstratorCard pane={pane} lifted={lifted} onClick={function () { return focusPane(pane.id); }} onMouseEnter={function () {
                        hoveredPaneIdRef.current = pane.id;
                        setHoveredPaneId(pane.id);
                        promoteAndResume(pane.id);
                        applyPaneScroll(paneIndex);
                        refreshRevealRect(pane.id, scrollOffsetForPaneIndex(paneIndex));
                    }} onMouseLeave={function () {
                        if (hoveredPaneIdRef.current === pane.id) {
                            hoveredPaneIdRef.current = null;
                            setHoveredPaneId(null);
                            setRevealRect(null);
                        }
                    }}/>
                </div>);
            })}
          </div>

          {overviewChrome}
        </>)}

      {overviewReturnButton}
    </div>);
}
//#endregion 🎪️DemonstratorLanding
(0, runtime_1.mountUiRoot)(document.getElementById("root"), <DemonstratorLanding />);
