/** 🎛️ The `InteractionRepl` component and its viewport.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { Button, borderNormalClass, canvasHostRootClass, Checkbox, cn, CanvasPickMenu, editorShellRootClass, floatingFieldSurfaceClass, floatingMenuSurfaceClass, floatingPaneAsideClass, floatingTagClass, floatingTagOffClass, floatingTagOnClass, Input, LevelProvider, MenuItem, Select, SelectContent, SelectionMarquee, SelectItem, SelectTrigger, SelectValue, WINDOW_SEARCH_USER, type ReactNode } from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { canvasPickTargetKey } from "@semio-tech/framework";

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
/** 🪩️ Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl(props: InteractionReplProps): ReactNode {
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
        <aside className={cn(floatingPaneAsideClass, fillHost ? "max-h-[45%] w-full shrink-0 border-l-0 border-t" : "w-layout-floating-menu-lg shrink-0")} style={asideStyle} data-level="pane">
        <LevelProvider level="pane">
          <strong className="text-sm font-semibold">Editor</strong>
          <div className="flex flex-wrap gap-half">
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
          </div>
          <div className={cn("relative grid overflow-visible", floatingFieldSurfaceClass)} data-level="menu">
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
              className="col-start-1 row-start-1 border-0 bg-transparent pr-large shadow-none focus-visible:ring-0"
            />
            <Button
              id="cad.replSuggestionsToggle"
              type="button"
              variant="outline"
              icon={interactionMenuOpen ? "chevron-up" : "chevron-down"}
              className={cn("col-start-1 row-start-1 z-[1] mr-single size-medium justify-self-end self-center p-0 text-2xs", interactionMenuOpen && "bg-accent text-accent-foreground")}
              onMouseDown={(e) => e.preventDefault()}
              onClick={() => {
                setInteractionMenuOpen((open) => !open);
                cmdRef.current?.focus();
              }}
              aria-label={WINDOW_SEARCH_USER.suggestionsAria}
            />
            {completionSuffix ? (
              <div aria-hidden className="text-element pointer-events-none col-start-1 row-start-1 overflow-hidden pr-large pl-double py-tiny text-sm leading-normal whitespace-pre">
                <span className="text-transparent">{cmdLine}</span>
                <span className="text-muted-foreground">{completionSuffix}</span>
              </div>
            ) : null}
            {interactionMenuOpen ? (
              <div
                role="menu"
                aria-label={WINDOW_SEARCH_USER.suggestionsAria}
                onPointerDown={(e) => e.stopPropagation()}
                className={cn("absolute top-[calc(100%+var(--spacing-double))] right-0 z-[3] max-h-layout-floating-menu-sm w-layout-floating-menu-md max-w-[calc(100vw-var(--size-xl))] overflow-y-auto p-single", floatingMenuSurfaceClass)}
                data-level="menu"
              >
                {interactionMatches.length ? (
                  interactionMatches.map((suggestion) => (
                    <MenuItem key={`${suggestion.kind}:${suggestion.key}:${suggestion.detail}`} onClick={() => runSuggestion(suggestion)}>
                      <div className="flex items-center gap-single">
                        <span className={cn(floatingTagClass, floatingTagOffClass, "inline-flex min-w-6 items-center justify-center px-half py-0 text-2xs font-bold uppercase")}>{suggestion.key}</span>
                        <span>{suggestion.label}</span>
                      </div>
                      {suggestion.detail ? <div className="text-muted-foreground text-2xs">{suggestion.detail}</div> : null}
                    </MenuItem>
                  ))
                ) : (
                  <div className="text-muted-foreground px-single py-half text-xs">{WINDOW_SEARCH_USER.noMatches}</div>
                )}
              </div>
            ) : null}
          </div>
          {asideExtra}
          {onDocumentModelChange ? (
            <SelectionAttributesPane
              model={documentModel.model}
              activeModelDefinitionId={activeModelDefinitionId ?? defaultModelDefinitionId()}
              selection={displayedSelectionTargets}
              selectionCount={displayedSelectionTargets.length}
              onModelChange={onDocumentModelChange}
            />
          ) : null}
          <div className="flex flex-col gap-single text-xs">
            {hideModelDefinitionControls ? null : (
              <>
                <label className="flex flex-col gap-half">
                  <span>Model definition</span>
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
                    <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
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
                </label>
                <span className="text-muted-foreground">
                  {modelDefinitionScope.typologies.length} kind{modelDefinitionScope.typologies.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.interactions.length} tool{modelDefinitionScope.interactions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.attributeDefinitions.length} attribute{modelDefinitionScope.attributeDefinitions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.propertyDefinitions.length} propert{modelDefinitionScope.propertyDefinitions.length === 1 ? "y" : "ies"}
                  {" · "}
                  {modelDefinitionScope.statDefinitions.length} stat{modelDefinitionScope.statDefinitions.length === 1 ? "" : "s"}
                </span>
                {transfersFrom.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer from</span>
                    <Select
                      id="cad.transferFromPicker"
                      key={transfersFromResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersFromResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                        <SelectValue placeholder="Select incoming transformation…" />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersFrom.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </label>
                ) : null}
                {transfersTo.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer to</span>
                    <Select
                      id="cad.transferToPicker"
                      key={transfersToResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersToResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                        <SelectValue placeholder="Select outgoing transformation…" />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersTo.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </label>
                ) : null}
              </>
            )}
            {!isShapeModelDefinition(activeModelDefinitionId) ? (
              <span className="text-muted-foreground">
                {viewObjectCount} object{viewObjectCount === 1 ? "" : "s"}
              </span>
            ) : null}
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={primitiveShowGroupState} ariaLabel="Show all primitives" onEnabledChange={(enabled) => setFilterPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled))} />
              Primitives · Show
            </label>
            <div role="group" aria-label="Show primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`show-primitive-${kind}`} className={cn(floatingTagClass, filterPrimitiveToggles[kind] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                  <Checkbox
                    checked={filterPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      setFilterPrimitiveToggles((prev) => ({ ...prev, [kind]: e.target.checked }));
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={primitiveFilterGroupState}
                ariaLabel="Filter all primitives"
                onEnabledChange={(enabled) => {
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
              />
              Primitives · Filter
            </label>
            <div role="group" aria-label="Filter primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`filter-primitive-${kind}`} className={cn(floatingTagClass, selectionPrimitiveToggles[kind] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                  <Checkbox
                    checked={selectionPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      const checked = e.target.checked;
                      setSelectionPrimitiveToggles((prev) => ({ ...prev, [kind]: checked }));
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                      if (!checked) {
                        applySelectionPrune((prev) => replPruneSelectionByPrimitive(prev, kind));
                      }
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={typologyShowGroupState} ariaLabel="Show all typologies" onEnabledChange={(enabled) => setFilterTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled))} />
              Typologies · Show
            </label>
            <div role="group" aria-label="Show typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`show-${typology.id}`} className={cn(floatingTagClass, filterTypologyToggles[typology.id] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                    <Checkbox
                      checked={filterTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        setFilterTypologyToggles((prev) => ({ ...prev, [typology.id]: e.target.checked }));
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={typologySelectionGroupState}
                ariaLabel="Select all typologies"
                onEnabledChange={(enabled) => {
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
              />
              Typologies · Selection
            </label>
            <div role="group" aria-label="Selection typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`select-${typology.id}`} className={cn(floatingTagClass, selectionTypologyToggles[typology.id] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                    <Checkbox
                      checked={selectionTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        const checked = e.target.checked;
                        setSelectionTypologyToggles((prev) => ({ ...prev, [typology.id]: checked }));
                        setSelectionMenu(null);
                        setHoveredPickKey(null);
                        if (!checked) {
                          applySelectionPrune((prev) => replPruneSelectionByTypology(prev, documentModel.model, activeModelDefinitionId, typology.id));
                        }
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
          </div>
          <div className="text-muted-foreground text-xs">
            {interactionId ? (
              <>
                Interaction <code className="text-foreground">{interactionId}</code> · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            ) : (
              <>
                No interaction selected · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            )}
          </div>
          <div className="border-border text-xs border-t pt-single">
            <strong className="text-foreground">Last response</strong>
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
          </div>
        </LevelProvider>
        </aside>
      ) : null}
    </div>
  );
}

/** 🪟️ Canvas-only {@link InteractionRepl} (no model-definition aside); full host props and `on*` callbacks. */
export function InteractionReplViewport(props: InteractionReplProps): ReactNode {
  return <InteractionRepl {...props} showAside={false} fillHost />;
}
// #endregion 🪩️Repl
