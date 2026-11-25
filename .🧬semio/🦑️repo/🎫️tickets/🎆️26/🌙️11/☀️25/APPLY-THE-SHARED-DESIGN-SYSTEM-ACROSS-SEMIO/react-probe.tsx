import "@semio-tech/ui-styling/🖌️ui.css";
import { createRoot } from "react-dom/client";
import { WindowChrome } from "@semio-tech/ui/elements/WindowChrome";
import { LevelProvider } from "@semio-tech/ui/elements/Surface";
import { IconShotFrame } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🟦️.tsx";

createRoot(document.getElementById("probe")!).render(
  <LevelProvider level="pane">
    <WindowChrome level="pane" bodySurfaceLevel="window" bodySurfaceFill="surface" style={{width: 240, height: 160}} titleChips={<span style={{width: 72}}>Title</span>} body={<p>Content</p>} />
    {(["rectangle", "ellipse"] as const).map(shape => (
      <div key={shape} style={{position: "relative", width: 240, height: 160}}>
        <IconShotFrame background="var(--background)" width={240} height={160} shape={shape} />
      </div>
    ))}
  </LevelProvider>,
);
