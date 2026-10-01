import { artifactFrontierIsGenesisForV1, artifactFrontierIsEditedForV1, decodePackValue, type DocumentScope, type ArtifactFrontier } from "@semio-tech/framework-os";
import type { ServiceMountedViewV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx";
import React from "react";
import { GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, gisMapInferencePortAffordancesV1, gisMapInferencePortRoleV1, gisMapInferencePortTerminalV1, projectGisMapInferencePreviewOverlayV1, parseGisMapInferencePortStatusV1, type GisMapInferencePortStatusV1 } from "../🧬️schema/🟦️.ts";
import type { InferencePortUiAction, InstalledServicePresentationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx";
/** ♿ Bilingual host-owned inference port. Work in flight announces politely, every terminal
 * asserts, progress is a real `<progress>` with the server's own bounded counters, and Cancel and
 * Approve are ordinary keyboard-reachable buttons — Approve exists only while a proposal is
 * actually offered and no cancel has been requested. The rendered text is the complete UI payload:
 * no job transport, origin, path, receipt, proposal body or user identity appears, and nothing here
 * is persisted into the document. Focus moves to the region when it opens and returns to whatever
 * held it before when it closes. */
export function InferencePortPanel({
  status,
  locale,
  onAction,
}: {
  readonly status: GisMapInferencePortStatusV1;
  readonly locale: "en" | "de";
  readonly onAction: (action: InferencePortUiAction) => void;
}) {
  const headingRef = React.useRef<HTMLHeadingElement | null>(null);
  React.useEffect(() => {
    const restore = document.activeElement;
    headingRef.current?.focus();
    return () => {
      if (restore instanceof HTMLElement && restore.isConnected) restore.focus();
    };
  }, []);
  const control = GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1;
  const phaseText = GIS_MAP_INFERENCE_PORT_TEXT_V1[status.phase][locale];
  const liveText = status.code === null ? phaseText : `${phaseText} ${GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1[status.code][locale]}`;
  const role = gisMapInferencePortRoleV1(status.phase);
  const terminal = gisMapInferencePortTerminalV1(status.phase);
  const chrome = gisMapInferencePortAffordancesV1(status);
  const overlay = status.preview && chrome.overlay ? projectGisMapInferencePreviewOverlayV1(status.preview) : null;
  return (
    <section aria-label={control.heading[locale]} data-semio-inference-port={status.phase}>
      <h2 ref={headingRef} tabIndex={-1}>{control.heading[locale]}</h2>
      <p role={role} aria-live={role === "status" ? "polite" : "assertive"}>{liveText}</p>
      {status.preview ? (
        <dl data-semio-inference-preview={status.preview.regionId}>
          <dt>{control.region[locale]}</dt><dd>{status.preview.regionId}</dd>
          <dt>{control.longitude[locale]}</dt><dd>{status.preview.ring[0][0]}–{status.preview.ring[2][0]}</dd>
          <dt>{control.latitude[locale]}</dt><dd>{status.preview.ring[0][1]}–{status.preview.ring[2][1]}</dd>
        </dl>
      ) : null}
      {overlay ? (
        <svg data-semio-inference-overlay={overlay.regionId} viewBox={overlay.viewBox} role="img" aria-label={control.overlay[locale]}>
          <title>{control.overlay[locale]}</title>
          <path d={overlay.path} fill="currentColor" fillOpacity="0.18" stroke="currentColor" strokeWidth="1.5" />
        </svg>
      ) : null}
      {status.total > 0 && !terminal ? <progress aria-label={control.progress[locale]} value={status.completed} max={status.total} /> : null}
      {chrome.request ? <button type="button" onClick={() => onAction({ kind: "propose", payload: { requestId: crypto.randomUUID().replaceAll("-", "") } })}>{control.request[locale]}</button> : null}
      {chrome.cancel ? <button type="button" onClick={() => onAction({ kind: "cancel" })}>{control.cancel[locale]}</button> : null}
      {chrome.reject ? <button type="button" onClick={() => onAction({ kind: "cancel" })}>{control.reject[locale]}</button> : null}
      {chrome.approve ? <button type="button" onClick={() => onAction({ kind: "approve" })}>{control.approve[locale]}</button> : null}
      <button type="button" onClick={() => onAction({ kind: "close" })}>{control.close[locale]}</button>
    </section>
  );
}

/** 🌍 GIS owns status decoding, controls, geometry projection and bilingual presentation. */
export const GIS_INFERENCE_PRESENTATION_V1: InstalledServicePresentationV1 = {
  probe: mountedGisMapProbeV1,
  owner: "gis", serviceId: "s.gis.gismap.inference",
  render(payload, locale, onAction) { return <InferencePortPanel status={parseGisMapInferencePortStatusV1(payload)} locale={locale} onAction={onAction} />; },
};

export type MountedGisMapProbeV1 = Readonly<{
  scope: DocumentScope;
  clientInstanceId: string;
  activationGeneration: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
  rootKind: "tiled-map";
  regionIds: readonly string[];
}>;

/** 🔬️ Projects only public identity and GIS scene facts from the Shell's acknowledged retained
 * store. It has no dispatch, credential, receipt, grant, proposal or undo-handle surface. */
export function mountedGisMapProbeV1(source: ServiceMountedViewV1 | null): MountedGisMapProbeV1 | null {
  if (source === null) return null;
  const state = source.store.getState();
  if (
    state.revision < 1 ||
    state.revision !== source.uiRevision ||
    state.root === null ||
    !/^[0-9a-f]{64}$/u.test(source.activeCheckpointId) ||
    !/^[0-9a-f]{64}$/u.test(source.descriptorDigestV1) ||
    (!(artifactFrontierIsGenesisForV1(source.scope, source.frontier) || artifactFrontierIsEditedForV1(source.scope, source.frontier)) || source.frontier.lastCommitSeq > source.frontier.headEditOrdinal)
  ) return null;
  const root = state.nodes.get(state.root);
  if (root?.component.type !== "surface" || root.component.kind !== "tiled-map") return null;
  let decoded: unknown;
  try {
    decoded = decodePackValue(new Uint8Array(root.component.doc.bytes));
  } catch {
    return null;
  }
  if (decoded === null || typeof decoded !== "object" || Array.isArray(decoded)) return null;
  const regions = (decoded as Record<string, unknown>).regions;
  if (!Array.isArray(regions) || regions.length > 4_096) return null;
  const regionIds: string[] = [];
  for (const region of regions) {
    if (region === null || typeof region !== "object" || Array.isArray(region)) return null;
    const id = (region as Record<string, unknown>).id;
    if (typeof id !== "string" || id.length === 0 || new TextEncoder().encode(id).byteLength > 256 || /[\u0000-\u001f\u007f]/u.test(id) || regionIds.includes(id)) return null;
    regionIds.push(id);
  }
  regionIds.sort();
  return Object.freeze({
    scope: Object.freeze({ ...source.scope }),
    clientInstanceId: source.clientInstanceId,
    activationGeneration: source.activationGeneration,
    catalogGenerationId: source.catalogGenerationId,
    componentSha256: source.componentSha256,
    descriptorSha256: source.descriptorSha256,
    browserActorSha256: source.browserActorSha256,
    activeCheckpointId: source.activeCheckpointId,
    descriptorDigestV1: source.descriptorDigestV1,
    frontier: Object.freeze({ ...source.frontier, chainHash: Object.freeze([...source.frontier.chainHash]) }),
    uiRevision: state.revision,
    rootKind: "tiled-map",
    regionIds: Object.freeze(regionIds),
  });
}

