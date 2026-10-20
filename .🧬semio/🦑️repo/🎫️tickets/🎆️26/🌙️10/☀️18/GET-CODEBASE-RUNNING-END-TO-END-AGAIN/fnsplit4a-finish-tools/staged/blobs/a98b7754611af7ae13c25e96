/** 🔶️ The catalog phase of useArtifactOpening. */
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { useCallback, useMemo, useState } from "react";
import { type AppRef, type AppRole, AppRouter, type AppRouterManifest, type ArtifactDialect, artifactKindActivationOwner, dialectCoordinate, parseDialectCoordinate, parseSurfaceAppId, EMPTY_OPENING_PREFERENCES, foldOpeningPreferences, type OpeningConfigMutation, type OpeningPreferences, type ConflictResolution, type MergePolicy } from "@semio-tech/framework";
import { resolveArtifactOpeningRelay, type ResolvedArtifactOpeningRelay } from "@semio-tech/framework/backbone";
import { localProgramsV1 } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { EMPTY_APP_LABELS_OVERLAY } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { applyFrameworkLayoutSeed } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { parseHubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import { withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterDirectoryLaneAndLayout } from "../../📐️layout-hotkeys/🟦️.ts";
import type { PreparedArtifactOpeningTarget } from "../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import type { PendingAppChannelMethods } from "../🟢️shared/🟦️.ts";

/** 🔷 Builds catalog, shellState, dispatch, loadedPlugins of useArtifactOpening. */
export function useCatalogState({ core }: { readonly core: ShellCoreAfterDirectoryLaneAndLayout }) {
  const {
    catalog, shellState, dispatch, loadedPlugins, session, localPrograms, uiLocale, uiTerminology, selectedConflictId,
    extraWindowCounterRef, extraWindowInstancesRef, applyRemoteMergeRef, showTransientNoticeRef,
    retireDocumentAttachment, openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef,
    resolveArtifactOpeningWithActivationRef, openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef,
    setSpaceArtifactCreationUi, cancelSpaceArtifactCreationsForRuntime, openDocumentSessionsRef, captureDialogOrigin,
    captureEffectOwner, isCurrentEffectOwner, loadedPluginsRef, pluginSupervisorByIdRef, registry, hubPluginSource,
    installPlugin, installHubDocumentProgram, shellStateRef, pendingDocumentOpeningPublicationRef, openDocument,
    closeDocument
  } = core;
  //#region 🔖️SurfaceRoles
  /** 👁️✏️ `(dialect, role) -> AppRef[]`, contract freeze §3 — built fresh from every loaded plugin's
   * manifest. `AppRouter.build` is total: a plugin with a genuine authoring defect
   * (`surface.conflict`/`surface.contribution-not-permitted`) is excluded on its own and reported
   * through {@link AppRouter.pluginFaults}, so one bad manifest costs that plugin's surfaces, never
   * every route in the session (ticket 26/09/05/S-END-TO-END lane H). */
  const appRouter = useMemo(
    (): AppRouter =>
      AppRouter.build(
        localPrograms.map((entry): AppRouterManifest => ({
          pluginId: entry.handle.pluginId,
          apps: entry.manifest.apps as unknown as Record<string, unknown>[],
          artifactKinds: entry.manifest.artifactKinds,
          dependencies: entry.manifest.dependencies,
        })),
      ),
    [localPrograms, registry],
  );
  /** 🧯️ `pluginId -> the surface fault that excluded it from {@link appRouter}` — merged onto the
   * plugin's own status so an excluded plugin is visible instead of silently unroutable. */
  const routerFaultByPluginId = useMemo(() => new Map(appRouter.pluginFaults().map((fault) => [fault.scope.pluginId ?? "", fault] as const)), [appRouter]);
  const pluginLabelById = useMemo(() => new Map(localPrograms.map((entry) => [entry.handle.pluginId, entry.manifest.label || entry.handle.pluginId])), [localPrograms]);
  /** 🔤️ Every dialect any loaded app declares — read straight off `AppDefinition.dialect`
   * (contract freeze §1), never inferred from a surface id string. Feeds the Settings
   * `SettingsDefaultApps` table; `appRouter` itself has no public "every registered dialect"
   * accessor (by design — it's addressed one `(dialect, role)` pair at a time). */
  const knownDialects = useMemo((): readonly ArtifactDialect[] => {
    const byCoordinate = new Map<string, ArtifactDialect>();
    for (const entry of localPrograms) for (const app of entry.manifest.apps) if (app.dialect) byCoordinate.set(dialectCoordinate(app.dialect), app.dialect);
    return [...byCoordinate.values()];
  }, [cancelSpaceArtifactCreationsForRuntime, localPrograms]);

  /** 🎚️ Client-side fold of `os.config.opening` (contract freeze §4) — event-sourced the same way
   * the host materializes it (`foldOpeningPreferences`), never a mutated map. There is no host
   * readback call on {@link AppChannelClient} yet (only the two write commands), so this mirror is
   * advanced ONLY by this shell's own `setDefaultApp`/`clearDefaultApp` calls below — a pin made by
   * another session/shell is not reflected here until that gap closes. See `📓️w1-c-report.md`. */
  const [openingPreferences, setOpeningPreferences] = useState<OpeningPreferences>(EMPTY_OPENING_PREFERENCES);
  const pinnedAppFor = useCallback((dialect: ArtifactDialect, role: AppRole): AppRef | undefined => openingPreferences.defaults.find((entry) => dialectCoordinate(entry.dialect) === dialectCoordinate(dialect) && entry.role === role)?.app, [openingPreferences]);
  resolveArtifactOpeningRelayRef.current = (actionId, args) => (appRouter ? resolveArtifactOpeningRelay(actionId, args, appRouter, openingPreferences) : null);

  /**
   * 🎬️ The hub half of `os.open-artifact`: a session that has never loaded the kind's owner has no
   * router entry for it, so the sync relay throws `surface.unknown-dialect` — not because the artifact
   * is unopenable, but because nobody who could open it is resident. The owner comes from the catalog's
   * declared `on-artifact-kind:` rows ({@link artifactKindActivationOwner}), which exist without any
   * manifest; this installs it, rebuilds the router over the freshly loaded set (the `appRouter` memo
   * this closure captured is a render behind `installPlugin`'s dispatch) and resolves once more. One
   * install, one retry: a second miss is a real routing gap and surfaces as the original fault.
   */
  const installActivationOwnerAndResolve = useCallback(async (actionId: string, args: unknown): Promise<ResolvedArtifactOpeningRelay | null> => {
    const record = args && typeof args === "object" && !Array.isArray(args) ? (args as Readonly<Record<string, unknown>>) : undefined;
    const artifactRef = typeof record?.artifactRef === "string" ? record.artifactRef : "";
    const openedArtifactKind = (): string => (artifactRef.includes("#") ? parseSurfaceAppId(artifactRef).dialect : parseDialectCoordinate(artifactRef)).artifactKind;
    if (hubPluginSource !== null && record !== undefined && typeof record.spaceId === "string" && record.spaceId.length > 0) {
      let artifactKind: string;
      try {
        artifactKind = openedArtifactKind();
      } catch {
        throw new Error("opening.invalid-artifact-ref");
      }
      const program = await installHubDocumentProgram(artifactKind);
      if (program === null || program.catalogModule === undefined) throw new Error(`opening.hub-program-unavailable: ${artifactKind}`);
      const catalogPluginId = program.catalogModule.pluginId;
      const programId = program.handle.pluginId;
      const routed = record.pluginId === catalogPluginId ? { ...record, pluginId: programId } : record;
      const router = AppRouter.build([{ pluginId: programId, apps: program.manifest.apps as unknown as Record<string, unknown>[], artifactKinds: program.manifest.artifactKinds, dependencies: program.manifest.dependencies }]);
      const preferences: OpeningPreferences = { ...openingPreferences, defaults: openingPreferences.defaults.map((entry) => (entry.app.pluginId === catalogPluginId ? { ...entry, app: { ...entry.app, pluginId: programId } } : entry)) };
      return resolveArtifactOpeningRelay(actionId, routed, router, preferences);
    }
    const resolveNow = resolveArtifactOpeningRelayRef.current;
    try {
      return resolveNow(actionId, args);
    } catch (relayError) {
      let artifactKind = "";
      try {
        artifactKind = openedArtifactKind();
      } catch {
        throw relayError;
      }
      const owner = artifactKindActivationOwner(catalog, artifactKind) ?? (await hubPluginSource?.ownerOfDialect(artifactKind).catch(() => undefined));
      if (owner === undefined || loadedPluginsRef.current.some((entry) => entry.handle.pluginId === owner)) throw relayError;
      const outcome = await installPlugin(owner);
      if (outcome !== "loaded" && outcome !== "already-loaded") throw relayError;
      const installed = AppRouter.build(localProgramsV1(loadedPluginsRef.current).map((entry): AppRouterManifest => ({
        pluginId: entry.handle.pluginId,
        apps: entry.manifest.apps as unknown as Record<string, unknown>[],
        artifactKinds: entry.manifest.artifactKinds,
        dependencies: entry.manifest.dependencies,
      })));
      return resolveArtifactOpeningRelay(actionId, args, installed, openingPreferences);
    }
  }, [catalog, installPlugin, installHubDocumentProgram, openingPreferences, hubPluginSource]);
  resolveArtifactOpeningWithActivationRef.current = installActivationOwnerAndResolve;

  
  const pendingAppChannelFor = useCallback(
    (pluginId: string): PendingAppChannelMethods | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle as PendingAppChannelMethods | undefined,
    [loadedPlugins],
  );

  /** ⚖️ Finds a loaded plugin's real `PluginWasmHandle` by pluginId, for the merge-policy/conflict
   * pass-throughs below — unlike {@link pendingAppChannelFor}'s ad hoc cast, `setMergePolicy`/
   * `resolveConflict`/`readConflicts` are genuine, always-present `PluginWasmHandle` members now
   * (`adaptPluginHandle`, `PluginRuntime/🟦️.tsx`), so no feature-detection is needed. */
  const pluginHandleFor = useCallback((pluginId: string): PluginWasmHandle | undefined => loadedPlugins.find((entry) => entry.handle.pluginId === pluginId)?.handle, [loadedPlugins]);

  const dispatchSetDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole, program: AppRef) => {
      const app: AppRef = { ...program, pluginId: parseHubProgramIdV1(program.pluginId)?.pluginId ?? program.pluginId };
      const mutation: OpeningConfigMutation = { mutation: "setDefaultApp", dialect, role, app };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(program.pluginId)
        ?.setDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum, app.pluginId, app.appId)
        ?.catch((commandError) => undefined);
    },
    [pendingAppChannelFor],
  );
  const dispatchClearDefaultApp = useCallback(
    (dialect: ArtifactDialect, role: AppRole) => {
      const mutation: OpeningConfigMutation = { mutation: "clearDefaultApp", dialect, role };
      setOpeningPreferences((current) => foldOpeningPreferences([mutation], current));
      const roleNum = role === "editor" ? 1 : 0;
      void pendingAppChannelFor(session?.pluginId ?? "")
        ?.clearDefaultApp?.(dialect.artifactKind, dialect.standard, dialect.subset, roleNum)
        ?.catch((commandError) => undefined);
    },
    [pendingAppChannelFor, session?.pluginId],
  );

  /** 💾️ Persists `os.config.merge-policy` through the `🛡️change-merge-policy` config triad's own
   * event-sourced fold (local, always works), then genuinely forwards `AppCommand::SetMergePolicy`
   * (contract freeze §C8) through `PluginWasmHandle.setMergePolicy` — a real handle member now (see
   * {@link pluginHandleFor}'s doc), so a session-less/plugin-not-loaded guard is the only thing
   * standing between this and the wire call; a failed dispatch surfaces loudly via the `.catch`
   * (never a silent optional-chain no-op). */
  const dispatchSetMergePolicy = useCallback(
    (policy: MergePolicy) => {
      dispatch({ type: "SET_MERGE_POLICY", value: policy });
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin.setMergePolicy(session.instanceId, policy).catch((commandError) => undefined);
    },
    [pluginHandleFor, session],
  );

  /** 🤝️ `📌️ChromePanels`' Conflicts panel Accept/Discard — forwards `AppCommand::ResolveConflict`
   * (contract freeze §C6/§C8/§C9) through `PluginWasmHandle.resolveConflict`; the local roster is
   * replaced from the `Conflicts` frame the reply itself batches (contract freeze §C6
   * `resolve_conflict`: "Returns the authoritative `MergeReport` + `Conflicts` frames"), never
   * optimistically, since `resolve_conflict` can itself reject (Quarantined+Accept still enforces
   * Fatal — that path returns no `Conflicts` frame, so the roster is left untouched). */
  const dispatchResolveConflict = useCallback(
    (conflictId: string, resolution: ConflictResolution) => {
      if (!session) return;
      const plugin = pluginHandleFor(session.pluginId);
      if (!plugin) return;
      void plugin
        .resolveConflict(session.instanceId, conflictId, resolution)
        .then((result) => {
          if (result.conflicts) dispatch({ type: "SET_CONFLICTS", value: result.conflicts });
        })
        .catch((commandError) => undefined);
    },
    [pluginHandleFor, session],
  );

  const publishPreparedArtifactOpening = useCallback((target: PreparedArtifactOpeningTarget, receipt?: DocumentOpeningReceiptV1): void => {
    const { session: nextSession, dialect, role } = target;
    const seeded = applyFrameworkLayoutSeed(nextSession.app.defaultLayout, withLocalizedWindowKindLabels(nextSession.app.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
    extraWindowInstancesRef.current = seeded.extraInstances;
    extraWindowCounterRef.current = seeded.extraInstances.length;
    pendingDocumentOpeningPublicationRef.current = receipt === undefined ? null : { receipt, plugin: target.plugin, session: nextSession };
    dispatch({ type: "SET_SESSION", value: nextSession });
    dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
    dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
    dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
    try {
      void (target.plugin as PendingAppChannelMethods).openArtifact?.(canonicalSurfaceId(dialect, role), role === "editor" ? 1 : 0, nextSession.pluginId, nextSession.app.id)
        ?.catch((commandError) => undefined);
    } catch (commandError) {
    }
  }, [uiLocale, uiTerminology]);
  return { shellStateRef, captureEffectOwner, captureDialogOrigin, isCurrentEffectOwner, loadedPluginsRef, installPlugin, publishPreparedArtifactOpening, loadedPlugins, openArtifactWithAppRefRef, spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, resolveArtifactOpeningWithActivationRef, openDocument, closeDocument, retireDocumentAttachment, openReadySpaceArtifactCreationRef, appRouter, knownDialects, pinnedAppFor, pluginLabelById, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp, shellState, selectedConflictId, dispatch, dispatchResolveConflict, session, pluginSupervisorByIdRef, showTransientNoticeRef, applyRemoteMergeRef, routerFaultByPluginId, dispatchSetMergePolicy };
}
