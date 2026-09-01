// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/TextEditorHost.stories.tsx
// Specs: Host the framework renderer's `TextEditorHost` against the real prebuilt `framework/editor/rs`
// `EditorSession` WASM engine.
// Summary: A debug-readout host mounts `TextEditorHost` against a `TextEditorScene`; `parameters.wasm: ["editor"]`
// gates first paint on `WASM_LOADERS.editor` (see `.storybook/preview.tsx`) resolving.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useState, type ReactElement } from "react";

import { TextEditorHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, TextEditorScene, UiComponentSceneNode } from "@semio-tech/framework";

//#region SceneFixtures
const JACK_SCENE: TextEditorScene = {
  buffer: "MATCH (a:Piece) RETURN a.name",
  language: "jack",
  tokensJson: JSON.stringify([
    { class: "keyword", start: 0, end: 5 },
    { class: "ident", start: 7, end: 8 },
  ]),
};

const DIAGNOSTIC_SCENE: TextEditorScene = {
  buffer: "hello\nworld",
  language: "plain",
  diagnosticsJson: JSON.stringify([{ start: 0, end: 5, severity: "warning", message: "example diagnostic" }]),
};
//#endregion SceneFixtures

//#region StoryHost
function TextEditorStoryHost({ scene, controllerId, surfaceId }: { readonly scene: TextEditorScene; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "text-editor", textEditor: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <TextEditorHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="text-editor-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/TextEditorHost",
  component: TextEditorStoryHost,
  parameters: { layout: "fullscreen", wasm: ["editor"] },
  tags: ["autodocs"],
} satisfies Meta<typeof TextEditorStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

export const JackDocument: Story = {
  args: { scene: JACK_SCENE, controllerId: "writer-play", surfaceId: "writer.play.editor" },
};

export const WithDiagnostics: Story = {
  args: { scene: DIAGNOSTIC_SCENE, controllerId: "writer-play", surfaceId: "writer.play.editor-diagnostics" },
};
