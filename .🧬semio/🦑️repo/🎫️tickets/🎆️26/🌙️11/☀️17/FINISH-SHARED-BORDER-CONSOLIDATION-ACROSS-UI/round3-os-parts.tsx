import * as React from "react";
import { useEffect, useRef } from "react";
import { Button, Footer, Layout, Mode, Navbar, SharedBorderRow, SharedBorderRowCell, Window, uiDataLabel } from "@semio-tech/ui-react";
import { NoticeActionRow, NoticeCell, NoticeStack } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧱️notice-bands/🟦️.tsx";
import { LocalFolderReconnectBand } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📎️local-folders/🟦️.tsx";
import { StagedReferenceField } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import { TimeTravelWindowIndicator } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { PresencePeersOverlay } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx";
import { PixelEditingOverlay } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx";
import editingFixture from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎚️paint2d-editing/🔣️.json";

import { StagedActionDialog } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/🟦️.tsx";

const noop = () => {};
const Shell = ({ children, subfooter }: { children?: React.ReactNode; subfooter?: React.ReactNode }) => (
  <div style={{ height: "100vh", position: "relative" }} className="semio-scope">
    <Layout navbar={<Navbar items={[{ key: "t", content: <b>Application</b> }]} />} footer={<Footer items={[{ key: "s", content: "Ready" }]} />} subfooter={subfooter} canvas={<div className="h-full" />} />
    {children}
  </div>
);
const stalls = [{ id: "s1", text: "Command Extrude is slow" }, { id: "s2", text: "Command Boolean is slow" }];
const installRow = <NoticeActionRow role="status" data-semio-plugin-install="" text="Installing plugin cad" action={{ label: "Cancel", onPress: noop }} actionProps={{ "data-semio-plugin-install-cancel": "" }}><progress className="ml-single" value={40} max={100} /></NoticeActionRow>;
const stallRows = <SharedBorderRow orientation="vertical" role="status" data-semio-command-stall="">{stalls.map((s) => <NoticeActionRow key={s.id} data-semio-command-stall-id={s.id} text={s.text} action={{ label: "Cancel", onPress: noop }} />)}</SharedBorderRow>;
const checkin = <SharedBorderRow role="status" data-semio-checkin-band="running" className="bg-menu text-sm"><NoticeCell className="flex-row items-center"><span>Checking in 3 of 9</span></NoticeCell><SharedBorderRowCell><progress value={3} max={9} /></SharedBorderRowCell><Button icon="x" type="button" variant="outline">Abort</Button></SharedBorderRow>;
const bands = (children: React.ReactNode) => <div data-semio-bottom-bands="" className="flex w-full flex-col items-stretch">{children}</div>;
const props = { documentJson: '{"layers":[{"kind":"pixel","id":"p","width":16,"height":16}]}', assetsJson: "{}", assetExtentsJson: "{}", selectionJson: '["p"]', activeUtility: "select", brushSize: 1, brushOpacity: 1, brushColor: "#000000", brushHardness: 1, paintTarget: "pixels" as const, maskValue: 255, fillTolerance: 24, dispatch: noop, onWheel: noop, onCameraChange: noop, editingOptions: { ...editingFixture.initial, filterKind: "brightness" } } as never;
function Paint({ more }: { more?: boolean }) {
  const host = useRef<HTMLDivElement>(null), camera = useRef({ x: 0, y: 0, zoom: 1 });
  useEffect(() => { const d = host.current?.querySelector("details"); if (d) d.open = !!more; }, [more]);
  return <div ref={host} data-border-probe-art="" style={{ position: "fixed", inset: 0 }} className="bg-muted"><PixelEditingOverlay {...(props as object)} camera={camera} container={host} /></div>;
}
const stagedHost = (locale: "en" | "de") => { const view = { locale, controls: null, pending: false, executing: false, closing: false, error: "Could not complete the action", ready: true }; return { subscribe: () => noop, snapshot: () => view, cancel: noop, execute: noop } as never; };
export const r3Parts: Record<string, () => React.ReactElement> = {
  "r4-staged-dialog": () => <div className="semio-scope"><StagedActionDialog host={stagedHost("en")} /></div>,
  "r4-staged-dialog-de": () => <div className="semio-scope"><StagedActionDialog host={stagedHost("de")} /></div>,
  "r3-bands-top": () => <Shell><NoticeStack data-semio-notice-stack="install" className="top-[var(--navbar-band-height)] left-1/2 max-w-[90vw] -translate-x-1/2 bg-menu">{installRow}{stallRows}</NoticeStack></Shell>,
  "r3-bands-stall-only": () => <Shell><NoticeStack className="top-[var(--navbar-band-height)] left-1/2 max-w-[90vw] -translate-x-1/2 bg-menu">{stallRows}</NoticeStack></Shell>,
  "r3-route-admission": () => <Shell><NoticeStack orientation="horizontal" role="status" className="bottom-[var(--footer-band-height)] left-1/2 z-40 max-w-[90vw] -translate-x-1/2"><NoticeCell>Open the hub to continue</NoticeCell><Button icon="arrow-right" type="button" variant="outline">Open hub</Button></NoticeStack></Shell>,
  "r3-subfooter": () => <Shell subfooter={bands(<><LocalFolderReconnectBand label="Folder" message="This document was attached to the folder puzzles." reconnect="Reconnect folder" forget="Forget folder" busy={false} onReconnect={noop} onForget={noop} />{checkin}</>)} />,
  "r3-subfooter-busy": () => <Shell subfooter={bands(<LocalFolderReconnectBand label="Folder" message="This document was attached to the folder puzzles." reconnect="Reconnect folder" forget="Forget folder" busy={true} onReconnect={noop} onForget={noop} />)} />,
  "r3-corner-trigger": () => <Shell><button type="button" className="fixed bottom-[var(--footer-band-height)] right-0 z-50 size-6 border-l border-t bg-background/80">⋯</button></Shell>,
  "r3-staged-reference": () => <div style={{ padding: 0, width: 360 }}><StagedReferenceField id="r" value={["a-1", "b-2", "c-3"]} many onChange={noop} /></div>,
  "r3-staged-reference-empty": () => <div style={{ width: 360 }}><StagedReferenceField id="r" value={[]} many onChange={noop} /></div>,
  "r3-paint2d": () => <Paint />,
  "r3-paint2d-more": () => <Paint more />,
  "r3-nodegraph-peers": () => <div data-border-probe-art="" style={{ position: "fixed", inset: 0 }} className="bg-muted"><PresencePeersOverlay peers={[{ clientId: "1", name: "Ada", selectionCount: 2 }, { clientId: "2", name: "Grace", selectionCount: 0 }] as never} /></div>,
  "r3-time-travel": () => <div style={{ height: 500 }}><Mode mobile={innerWidth < 768} windows={[{ id: "a", title: uiDataLabel("a"), iconId: "app-window", children: <div />, controls: <TimeTravelWindowIndicator stage={"editing" as never} description="History editing" /> }] as never} activeWindowId="a" /></div>,
};
