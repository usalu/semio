/** 🟢️ The effective mode layout phase of usePanelHosting. */
import { useCallback, useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { windowLayoutWindowIdsV1 } from "@semio-tech/framework";
import { type WindowLayoutNode } from "@semio-tech/ui-react";
import { LAYOUT_CHANGE_SETTLE_MS, classifyWindowLayoutChange, resolveFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { shellLabelTextV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useBrandState } from "../🔶️brand/🟦️.tsx";

/** 🔵 Builds effectiveModeLayout, handleActiveWindowChange, layoutChangeSettleTimeoutRef, layoutChangeClassificationRef of usePanelHosting. */
export function useEffectiveModeLayoutState({ shellLayout, session, appLabelsOverlay, uiTerminology, uiLocale, effectiveModeLayoutRef, mountedWindowIdsRef, sessionRef, unmountedSkippedWindowBodiesRef, refreshUi, dropForRetiredInstance, dispatch, noteShellCommand }: Pick<ReturnType<typeof useBrandState>, "shellLayout" | "session" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "effectiveModeLayoutRef" | "mountedWindowIdsRef" | "sessionRef" | "unmountedSkippedWindowBodiesRef" | "refreshUi" | "dropForRetiredInstance" | "dispatch" | "noteShellCommand">) {
  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session ? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale).modeLayout : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, session, shellLayout, uiTerminology, uiLocale],
  );
  effectiveModeLayoutRef.current = effectiveModeLayout;
  mountedWindowIdsRef.current = useMemo(() => windowLayoutWindowIdsV1(effectiveModeLayout), [effectiveModeLayout]);
  useEffect(() => {
    const live = sessionRef.current;
    const skipped = unmountedSkippedWindowBodiesRef.current;
    if (!live || skipped.size === 0) return;
    const mounted = mountedWindowIdsRef.current;
    const windowBodies = [...skipped].filter(([windowId]) => mounted.has(windowId)).map(([, bodyKey]) => bodyKey);
    if (windowBodies.length === 0) return;
    unmountedSkippedWindowBodiesRef.current = new Map([...skipped].filter(([windowId]) => !mounted.has(windowId)));
    void refreshUi(live, { kind: "partial", windowBodies }).catch((error) => {
      if (!dropForRetiredInstance(live, "remounted window refresh", error)) console.error("[os-shell] remounted window refresh failed", error);
    });
  }, [dropForRetiredInstance, effectiveModeLayout, refreshUi]);

  const handleActiveWindowChange = useCallback(
    (value: string | null) => {
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value });
      if (value) noteShellCommand("shell.windowActivate", shellLabelTextV1("ui.shellCommand.windowActivate"), { windowId: value });
    },
    [noteShellCommand],
  );

  const layoutChangeSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const layoutChangeClassificationRef = useRef<"resize" | "rearrange" | null>(null);
  const layoutChangePreviousRef = useRef<WindowLayoutNode | null>(effectiveModeLayout);
  useEffect(() => {
    layoutChangePreviousRef.current = effectiveModeLayout;
  }, [effectiveModeLayout]);
  useEffect(
    () => () => {
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
    },
    [],
  );
  const handleModeLayoutChange = useCallback(
    (value: WindowLayoutNode) => {
      dispatch({ type: "SET_SHELL_LAYOUT", value });
      const classification = classifyWindowLayoutChange(layoutChangePreviousRef.current, value);
      layoutChangePreviousRef.current = value;
      if (classification) layoutChangeClassificationRef.current = classification;
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
      layoutChangeSettleTimeoutRef.current = setTimeout(() => {
        layoutChangeSettleTimeoutRef.current = null;
        const finalClassification = layoutChangeClassificationRef.current;
        layoutChangeClassificationRef.current = null;
        if (finalClassification === "resize") noteShellCommand("shell.windowResize", shellLabelTextV1("ui.shellCommand.windowResize"));
        else if (finalClassification === "rearrange") noteShellCommand("shell.windowMove", shellLabelTextV1("ui.shellCommand.windowMove"));
      }, LAYOUT_CHANGE_SETTLE_MS);
    },
    [noteShellCommand],
  );
  return { effectiveModeLayout, handleActiveWindowChange, handleModeLayoutChange };
}
