/** 🎛️ The `InteractionRepl` component and its viewport.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { Button, canvasHostRootClass, cn, CanvasPickMenu, editorShellRootClass, floatingPaneAsideClass, Input, LevelProvider, MenuItem, Select, SelectContent, SelectionMarquee, SelectItem, SelectTrigger, SelectValue, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, SharedBorderRowLabel, ToggleGroup, WINDOW_SEARCH_USER, type ReactNode } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { canvasPickTargetKey } from "@semio-tech/framework";
import { useCadReplLabels } from "../🗣️labels/🟦️.ts";

import { isShapeModelDefinition, defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { qualifiedTransformationId } from "@semio-tech/s-spatial-kernel-ts/geometry/transformation-spec";

import { cadFieldClass } from "../🎨️style/🟦️.ts";

import { SPATIAL_PRIMITIVE_KINDS, spatialPickCanvasRequest, spatialPickTargetKey, spatialToggleGroupFill, spatialTypologyToggleLabel } from "../🧲️geometry-targets/🎯️pick-model/🟦️.ts";

import { targetStyle } from "../🧲️geometry-targets/🎨️pick-styling/🟦️.ts";

import { InteractionCanvas, InteractionSelectionInvalidateBridge, InteractionSpatialView } from "../🪩️canvas/🟦️.tsx";
import { type InteractionReplProps, replNormalizeActionText, replPruneSelectionByPrimitive, replPruneSelectionByTypology, SpatialChromeMasterToggle, WorldOrbitProjectionSwitchPane } from "../🧾️repl-model/🟦️.tsx";
import { SelectionAttributesPane } from "../📋️repl-panes/🟦️.tsx";
import { useReplViewState, type ReplCoreAfter1 } from "./📺️view-state/🟦️.ts";
import { useReplSessionLifecycle, type ReplCoreAfter2 } from "./🎬️session-lifecycle/🟦️.ts";
import { useReplSelectionAndPointer, type ReplCoreAfter3 } from "./🎯️selection-pointer/🟦️.ts";
import { useReplCommandPalette, type ReplCoreAfter4 } from "./🧾️command-palette/🟦️.ts";
import { useReplEngagementAndKeys, type ReplCoreAfter5 } from "./🤝️engagement-keys/🟦️.ts";

// #region 🪩️Repl
/** 🧩️ One toggle section of the aside: a master checkbox cell over a wrapping multiple {@link ToggleGroup} of chips, merged into one vertical shared-border row so every seam is one hairline. */
function ReplToggleSection({ id, title, ariaLabel, masterAriaLabel, masterState, onMasterChange, entries, onChange }: {
  readonly id: string;
  readonly title: string;
  readonly ariaLabel: string;
  readonly masterAriaLabel: string;
  readonly masterState: Parameters<typeof SpatialChromeMasterToggle>[0]["state"];
  readonly onMasterChange: (enabled: boolean) => void;
  readonly entries: readonly { readonly key: string; readonly label: string; readonly enabled: boolean }[];
  readonly onChange: (next: readonly string[], previous: readonly string[]) => void;
}): ReactNode {
  if (!entries.length) return null;
  const value = entries.filter((entry) => entry.enabled).map((entry) => entry.key);
  return (
    <SharedBorderRow orientation="vertical">
      <SharedBorderRowField className="text-foreground flex-none font-semibold">
        <SpatialChromeMasterToggle state={masterState} ariaLabel={masterAriaLabel} onEnabledChange={onMasterChange} />
        {title}
      </SharedBorderRowField>
      <ToggleGroup id={id} kind="multiple" wrap aria-label={ariaLabel} value={value} onValueChange={(next) => onChange(next, value)} items={entries.map((entry) => ({ value: entry.key, icon: entry.enabled ? "check" : "square", text: entry.label }))} />
    </SharedBorderRow>
  );
}

/** 🪩️ Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl(props: InteractionReplProps): ReactNode {
  const labels = useCadReplLabels();
  const core = {} as ReplCoreAfter5;
  Object.assign(core, useReplViewState(props));
  Object.assign(core, useReplSessionLifecycle(core));
  Object.assign(core, useReplSelectionAndPointer(core));
  Object.assign(core, useReplCommandPalette(core));
  Object.assign(core, useReplEngagementAndKeys(core));
  const { activeModelDefinitionId, activeSelectionAccept, activeTransformGumballConfig, applySelectionPrune, asideExtra, asideStyle, autoFitBehavior, autoFitMeshes, canvasFrameloop, canvasOverrides, cmdLine, cmdRef, committedMeshesForView, completionSuffix, dispatchSelectionTargets, displayedSelectionTargets, documentModel, dragOverlayPoints, dragOverlayRect, dragSelection, effectiveSelectionKindToggles, engagementActionMode, externalPendingProjectionSpec, factoryFaceMeshesForView, fillHost, filterPrimitiveToggles, filterTypologyToggles, handleCanvasReady, handleProjectionSpecChange, handleTransformGumballCommit, handleTransformGumballPreview, handleTransformGumballPreviewEnd, hideModelDefinitionControls, hostPickingEnabled, hoveredPickKey, hoveredReferenceId, interactionId, interactionMatches, interactionMenuOpen, lr, mergedDisplay, modelDefinitionRevision, modelDefinitionScope, modelDefinitions, onApplyTransformation, onCameraNavigate, onDocumentModelChange, onGroundPickProp, onInputKeyDown, onReferenceHover, onReferenceRelocate, onReferenceSelect, onScenePointerMove, onSelectionRequest, onSpatialInteractionEvent, pickPlaneOn, pickSourceGeometry, pointerMoveActive, primitiveFilterGroupState, primitiveShowGroupState, projectionSpec, referenceRelocateActive, renderDisplayItem, revealedReferenceIds, rootStyle, rt, runSuggestion, runTransitionRow, sceneKindToggles, scopeTypologyIds, selectedPickKey, selectedPickKeys, selectedReferenceIds, selectionInvalidateKey, selectionMenu, selectionPrimitiveToggles, selectionTypologyToggles, setActiveModelDefinitionId, setCmdLine, setExternalPendingProjectionSpec, setFilterPrimitiveToggles, setFilterTypologyToggles, setHoveredPickKey, setInteractionMenuOpen, setModelDefinitionRevision, setSelectionMenu, setSelectionPrimitiveToggles, setSelectionTypologyToggles, setTransfersFromResetKey, setTransfersToResetKey, showAside, showPickLayer, snapshot, spatialViewOverrides, transfersFrom, transfersFromResetKey, transfersTo, transfersToResetKey, transitionRows, typologySelectionGroupState, typologyShowGroupState, viewFilterKindToggles, viewGeometry, viewObjectCount, viewSlots, viewTheme, worldReferences } = core;

  return (
    <div className={fillHost ? canvasHostRootClass : editorShellRootClass} data-level="base" style={rootStyle}>
      <div className="relative min-h-0 min-w-0 flex-1">
        <InteractionCanvas
          {...canvasOverrides}
          managedCamera={spatialViewOverrides?.cameraView !== undefined}
          frameloop={canvasFrameloop}
          className={cn("bg-transparent", canvasOverrides?.className)}
          onCanvasReady={handleCanvasReady}
          overlay={<WorldOrbitProjectionSwitchPane spec={projectionSpec} onSpecChange={handleProjectionSpecChange} />}
        >
          <InteractionSelectionInvalidateBridge selectionKey={selectionInvalidateKey} />
          <InteractionSpatialView
            previewKernel={rt.previewKernel()}
            snapshot={snapshot}
            onInteractionEvent={onSpatialInteractionEvent}
            onGroundPick={onGroundPickProp}
            onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
            pickEnabled={pickPlaneOn}
            geometry={viewGeometry}
            pickGeometry={pickSourceGeometry}
            committedMeshes={committedMeshesForView}
            factoryFaceMeshes={factoryFaceMeshesForView}
            transformGumballModel={documentModel.model}
            onTransformGumballPreview={handleTransformGumballPreview}
            onTransformGumballPreviewEnd={handleTransformGumballPreviewEnd}
            onTransformGumballCommit={handleTransformGumballCommit}
            activeModelDefinitionId={activeModelDefinitionId}
            modelDefinitionRevision={modelDefinitionRevision}
            displayModel={mergedDisplay}
            renderDisplayItem={renderDisplayItem}
            selectionAccept={hostPickingEnabled ? activeSelectionAccept : []}
            filterKindToggles={viewFilterKindToggles}
            sceneKindToggles={sceneKindToggles}
            selectionKindToggles={effectiveSelectionKindToggles}
            hoveredTargetKey={hoveredPickKey}
            selectedTargetKey={selectedPickKey}
            selectedTargetKeys={selectedPickKeys}
            hostSelectionEnabled={hostPickingEnabled}
            showPickLayer={showPickLayer}
            onSelectionRequest={onSelectionRequest}
            onCameraNavigate={onCameraNavigate}
            autoFitMeshes={autoFitMeshes}
            autoFitBehavior={autoFitBehavior}
            theme={viewTheme}
            slots={viewSlots}
            transformGumballConfig={activeTransformGumballConfig}
            transformGumballTargets={displayedSelectionTargets}
            worldReferences={worldReferences}
            selectedReferenceIds={selectedReferenceIds}
            hoveredReferenceId={hoveredReferenceId}
            revealedReferenceIds={revealedReferenceIds}
            referenceRelocateActive={referenceRelocateActive}
            onReferenceSelect={onReferenceSelect}
            onReferenceHover={onReferenceHover}
            onReferenceRelocate={onReferenceRelocate}
            {...spatialViewOverrides}
            projectionSpec={projectionSpec}
            externalPendingProjectionSpec={externalPendingProjectionSpec}
            onExternalPendingProjectionSpecClear={() => setExternalPendingProjectionSpec(null)}
            onProjectionSpecChange={handleProjectionSpecChange}
          />
        </InteractionCanvas>
        {dragSelection && dragOverlayRect ? (
          dragSelection.method === "rectangle" ? (
            <SelectionMarquee
              className="z-[4]"
              coverage={dragSelection.coverage}
              shape="rect"
              rect={{
                x: Math.min(dragOverlayPoints[0]?.x ?? 0, dragOverlayPoints[1]?.x ?? 0),
                y: Math.min(dragOverlayPoints[0]?.y ?? 0, dragOverlayPoints[1]?.y ?? 0),
                width: Math.abs((dragOverlayPoints[1]?.x ?? 0) - (dragOverlayPoints[0]?.x ?? 0)),
                height: Math.abs((dragOverlayPoints[1]?.y ?? 0) - (dragOverlayPoints[0]?.y ?? 0)),
              }}
            />
          ) : (
            <SelectionMarquee className="z-[4]" coverage={dragSelection.coverage} shape="polygon" points={dragOverlayPoints} />
          )
        ) : null}
        {selectionMenu ? (
          <CanvasPickMenu
            request={spatialPickCanvasRequest(selectionMenu)}
            hoveredKey={hoveredPickKey}
            onHoverKey={(key) => setHoveredPickKey(key)}
            onPick={(target) => {
              const spatial = selectionMenu.targets.find((row) => spatialPickTargetKey(row) === canvasPickTargetKey(target));
              if (spatial) dispatchSelectionTargets([spatial], selectionMenu.modifiers, selectionMenu.point);
            }}
            onDismiss={() => setSelectionMenu(null)}
            renderRow={(target, active) => {
              const spatial = selectionMenu.targets.find((row) => spatialPickTargetKey(row) === canvasPickTargetKey(target));
              if (!spatial) return <span>{target.label}</span>;
              return (
                <>
                  <span className="mr-single inline-block size-2 rounded-xs" style={{ background: targetStyle(spatial, false, false).color }} />
                  <span className="text-muted-foreground">{target.domain}</span> <code className="text-foreground">{target.label}</code>
                </>
              );
            }}
          />
        ) : null}
      </div>
      {showAside ? (
        <SharedBorderRow as="aside" orientation="vertical" data-level="pane" className={cn(floatingPaneAsideClass, fillHost ? "max-h-[45%] w-full" : "w-layout-floating-menu-lg")} style={asideStyle}>
        <LevelProvider level="pane">
          <SharedBorderRowCell className="shrink-0">
            <strong className="text-sm font-semibold">{labels.editor}</strong>
          </SharedBorderRowCell>
          <SharedBorderRow orientation="vertical" className="min-h-0 flex-1 overflow-auto [&>*]:shrink-0">
            {transitionRows.length ? (
              <SharedBorderRow wrap>
                {transitionRows.map((row) => (
                  <Button
                    key={`${row.key}-${row.eventKind}-${row.label}`}
                    id={`cad.replTransition.${row.eventKind}.${row.key}`}
                    type="button"
                    variant="outline"
                    icon="arrow-right"
                    className="h-auto px-single py-half text-xs"
                    onClick={() => runTransitionRow(row)}
                  >
                    <span className="font-bold underline">{row.key}</span> {row.label}
                  </Button>
                ))}
              </SharedBorderRow>
            ) : null}
            <SharedBorderRow data-level="menu">
              <Input
                id="cad.replCommand"
                ref={cmdRef}
                type="text"
                autoComplete="off"
                spellCheck={false}
                value={cmdLine}
                onChange={(e) => {
                  setCmdLine(replNormalizeActionText(e.target.value, engagementActionMode));
                  if (interactionMenuOpen) setInteractionMenuOpen(true);
                }}
                onKeyDown={onInputKeyDown}
                placeholder={WINDOW_SEARCH_USER.actionPlaceholder}
                aria-label={WINDOW_SEARCH_USER.actionPlaceholder}
                className="bg-transparent"
              />
              <Button
                id="cad.replSuggestionsToggle"
                type="button"
                variant="outline"
                icon={interactionMenuOpen ? "chevron-up" : "chevron-down"}
                className={cn("flex-none p-0 text-2xs", interactionMenuOpen && "bg-accent text-accent-foreground")}
                onMouseDown={(e) => e.preventDefault()}
                onClick={() => {
                  setInteractionMenuOpen((open) => !open);
                  cmdRef.current?.focus();
                }}
                aria-label={WINDOW_SEARCH_USER.suggestionsAria}
              />
              {completionSuffix ? (
                <div aria-hidden className="text-element pointer-events-none absolute inset-y-0 left-0 right-medium overflow-hidden px-single py-single text-sm leading-normal whitespace-pre">
                  <span className="text-transparent">{cmdLine}</span>
                  <span className="text-muted-foreground">{completionSuffix}</span>
                </div>
              ) : null}
              {interactionMenuOpen ? (
                <div className="absolute inset-x-0 top-full z-[3]" onPointerDown={(e) => e.stopPropagation()}>
                  <SharedBorderRow role="menu" aria-label={WINDOW_SEARCH_USER.suggestionsAria} orientation="vertical" edges={["right", "bottom", "left"]} className={floatingPaneAsideClass} data-level="menu">
                    <SharedBorderRow orientation="vertical" className="max-h-layout-floating-menu-sm overflow-y-auto [&>*]:shrink-0">
                      {interactionMatches.length ? (
                        interactionMatches.map((suggestion) => (
                          <MenuItem key={`${suggestion.kind}:${suggestion.key}:${suggestion.detail}`} data-shared-border-cell="" className="rounded-none px-single py-single" onClick={() => runSuggestion(suggestion)}>
                            <div className="flex items-center gap-single">
                              <kbd className="text-muted-foreground inline-block min-w-6 text-center font-sans text-2xs font-bold uppercase">{suggestion.key}</kbd>
                              <span>{suggestion.label}</span>
                            </div>
                            {suggestion.detail ? <div className="text-muted-foreground text-2xs">{suggestion.detail}</div> : null}
                          </MenuItem>
                        ))
                      ) : (
                        <SharedBorderRowCell className="text-muted-foreground text-xs">{WINDOW_SEARCH_USER.noMatches}</SharedBorderRowCell>
                      )}
                    </SharedBorderRow>
                  </SharedBorderRow>
                </div>
              ) : null}
            </SharedBorderRow>
            {asideExtra ? <SharedBorderRowCell>{asideExtra}</SharedBorderRowCell> : null}
            {onDocumentModelChange ? (
              <SelectionAttributesPane
                model={documentModel.model}
                activeModelDefinitionId={activeModelDefinitionId ?? defaultModelDefinitionId()}
                selection={displayedSelectionTargets}
                selectionCount={displayedSelectionTargets.length}
                onModelChange={onDocumentModelChange}
              />
            ) : null}
            {hideModelDefinitionControls ? null : (
              <>
                <SharedBorderRow>
                  <SharedBorderRowLabel htmlFor="cad.modelDefinitionPicker-trigger">{labels.modelDefinition}</SharedBorderRowLabel>
                  <Select
                    id="cad.modelDefinitionPicker"
                    value={activeModelDefinitionId ?? defaultModelDefinitionId()}
                    onValueChange={(next) => {
                      setActiveModelDefinitionId(next || defaultModelDefinitionId());
                      setModelDefinitionRevision((r) => r + 1);
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                    }}
                  >
                    <SelectTrigger className={cn(cadFieldClass, "flex-1")}>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {modelDefinitions.map((row) => (
                        <SelectItem key={row.id} value={row.id}>
                          {row.label} ({row.id})
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </SharedBorderRow>
                <SharedBorderRowCell className="text-muted-foreground block text-xs">
                  {modelDefinitionScope.typologies.length} kind{modelDefinitionScope.typologies.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.interactions.length} tool{modelDefinitionScope.interactions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.attributeDefinitions.length} attribute{modelDefinitionScope.attributeDefinitions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.propertyDefinitions.length} propert{modelDefinitionScope.propertyDefinitions.length === 1 ? "y" : "ies"}
                  {" · "}
                  {modelDefinitionScope.statDefinitions.length} stat{modelDefinitionScope.statDefinitions.length === 1 ? "" : "s"}
                </SharedBorderRowCell>
                {transfersFrom.length ? (
                  <SharedBorderRow>
                    <SharedBorderRowLabel htmlFor="cad.transferFromPicker-trigger">{labels.transferFrom}</SharedBorderRowLabel>
                    <Select
                      id="cad.transferFromPicker"
                      key={transfersFromResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersFromResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "flex-1")}>
                        <SelectValue placeholder={labels.selectIncoming} />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersFrom.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </SharedBorderRow>
                ) : null}
                {transfersTo.length ? (
                  <SharedBorderRow>
                    <SharedBorderRowLabel htmlFor="cad.transferToPicker-trigger">{labels.transferTo}</SharedBorderRowLabel>
                    <Select
                      id="cad.transferToPicker"
                      key={transfersToResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersToResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "flex-1")}>
                        <SelectValue placeholder={labels.selectOutgoing} />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersTo.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </SharedBorderRow>
                ) : null}
              </>
            )}
            {!isShapeModelDefinition(activeModelDefinitionId) ? (
              <SharedBorderRowCell className="text-muted-foreground text-xs">
                {viewObjectCount} object{viewObjectCount === 1 ? "" : "s"}
              </SharedBorderRowCell>
            ) : null}
            <ReplToggleSection
              id="cad.replShowPrimitives"
              title="Primitives · Show"
              ariaLabel={labels.showPrimitives}
              masterAriaLabel="Show all primitives"
              masterState={primitiveShowGroupState}
              onMasterChange={(enabled) => setFilterPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled))}
              entries={SPATIAL_PRIMITIVE_KINDS.map((kind) => ({ key: kind, label: kind, enabled: filterPrimitiveToggles[kind] !== false }))}
              onChange={(next) => setFilterPrimitiveToggles((prev) => ({ ...prev, ...Object.fromEntries(SPATIAL_PRIMITIVE_KINDS.map((kind) => [kind, next.includes(kind)])) }))}
            />
            <ReplToggleSection
              id="cad.replFilterPrimitives"
              title="Primitives · Filter"
              ariaLabel={labels.filterPrimitives}
              masterAriaLabel="Filter all primitives"
              masterState={primitiveFilterGroupState}
              onMasterChange={(enabled) => {
                setSelectionPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled));
                setSelectionMenu(null);
                setHoveredPickKey(null);
                if (!enabled) {
                  applySelectionPrune((prev) => {
                    let next = prev;
                    for (const kind of SPATIAL_PRIMITIVE_KINDS) {
                      next = replPruneSelectionByPrimitive(next, kind);
                    }
                    return next;
                  });
                }
              }}
              entries={SPATIAL_PRIMITIVE_KINDS.map((kind) => ({ key: kind, label: kind, enabled: selectionPrimitiveToggles[kind] !== false }))}
              onChange={(next, previous) => {
                setSelectionPrimitiveToggles((prev) => ({ ...prev, ...Object.fromEntries(SPATIAL_PRIMITIVE_KINDS.map((kind) => [kind, next.includes(kind)])) }));
                setSelectionMenu(null);
                setHoveredPickKey(null);
                const removed = previous.filter((kind) => !next.includes(kind));
                if (removed.length) applySelectionPrune((prev) => removed.reduce((acc, kind) => replPruneSelectionByPrimitive(acc, kind as (typeof SPATIAL_PRIMITIVE_KINDS)[number]), prev));
              }}
            />
            <ReplToggleSection
              id="cad.replShowTypologies"
              title="Typologies · Show"
              ariaLabel={labels.showTypologies}
              masterAriaLabel="Show all typologies"
              masterState={typologyShowGroupState}
              onMasterChange={(enabled) => setFilterTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled))}
              entries={modelDefinitionScope.typologies.map((typology) => ({ key: typology.id, label: spatialTypologyToggleLabel(typology.id, typology.label), enabled: filterTypologyToggles[typology.id] !== false }))}
              onChange={(next) => setFilterTypologyToggles((prev) => ({ ...prev, ...Object.fromEntries(modelDefinitionScope.typologies.map((typology) => [typology.id, next.includes(typology.id)])) }))}
            />
            <ReplToggleSection
              id="cad.replSelectTypologies"
              title="Typologies · Selection"
              ariaLabel={labels.selectionTypologies}
              masterAriaLabel="Select all typologies"
              masterState={typologySelectionGroupState}
              onMasterChange={(enabled) => {
                setSelectionTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled));
                setSelectionMenu(null);
                setHoveredPickKey(null);
                if (!enabled) {
                  applySelectionPrune((prev) => {
                    let next = prev;
                    for (const typologyId of scopeTypologyIds) {
                      next = replPruneSelectionByTypology(next, documentModel.model, activeModelDefinitionId, typologyId);
                    }
                    return next;
                  });
                }
              }}
              entries={modelDefinitionScope.typologies.map((typology) => ({ key: typology.id, label: spatialTypologyToggleLabel(typology.id, typology.label), enabled: selectionTypologyToggles[typology.id] !== false }))}
              onChange={(next, previous) => {
                setSelectionTypologyToggles((prev) => ({ ...prev, ...Object.fromEntries(modelDefinitionScope.typologies.map((typology) => [typology.id, next.includes(typology.id)])) }));
                setSelectionMenu(null);
                setHoveredPickKey(null);
                const removed = previous.filter((typologyId) => !next.includes(typologyId));
                if (removed.length) applySelectionPrune((prev) => removed.reduce((acc, typologyId) => replPruneSelectionByTypology(acc, documentModel.model, activeModelDefinitionId, typologyId), prev));
              }}
            />
            <SharedBorderRowCell className="text-muted-foreground block text-xs">
              {interactionId ? (
                <>
                  Interaction <code className="text-foreground">{interactionId}</code> · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
                </>
              ) : (
                <>
                  No interaction selected · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
                </>
              )}
            </SharedBorderRowCell>
            <SharedBorderRowCell className="block text-xs">
              <strong className="text-foreground">{labels.lastResponse}</strong>
              <pre className="text-muted-foreground mt-half mb-0 max-h-layout-popover overflow-auto text-2xs">{lr ? JSON.stringify(lr, null, 2) : "—"}</pre>
              {snapshot.diagnostics.length ? (
                <ul className="text-muted-foreground m-0 list-inside list-disc text-2xs">
                  {snapshot.diagnostics.map((d, i) => (
                    <li key={`${d.code}-${i}`}>
                      [{d.severity}] {d.code}: {d.message}
                    </li>
                  ))}
                </ul>
              ) : null}
            </SharedBorderRowCell>
          </SharedBorderRow>
        </LevelProvider>
        </SharedBorderRow>
      ) : null}
    </div>
  );
}

/** 🪟️ Canvas-only {@link InteractionRepl} (no model-definition aside); full host props and `on*` callbacks. */
export function InteractionReplViewport(props: InteractionReplProps): ReactNode {
  return <InteractionRepl {...props} showAside={false} fillHost />;
}
// #endregion 🪩️Repl
