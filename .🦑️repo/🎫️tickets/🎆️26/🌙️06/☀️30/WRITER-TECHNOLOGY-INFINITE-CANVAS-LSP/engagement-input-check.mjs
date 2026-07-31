/** @emoji 🧪️ Writer play engagement.input compliance smoke. */
import { join } from "node:path";

const { CommandBus, enforcePlaygroundWindowEngagementInput } = await import(join(process.cwd(), "framework/product/playground/core/index.ts"));
const { WriterPlayController, WRITER_PLAY_WINDOW_KIND } = await import(join(process.cwd(), "writer/play/index.ts"));
const { createWriterDocument, writerDocumentToJson } = await import(join(process.cwd(), "writer/core/index.ts"));

const bus = new CommandBus();
const ctrl = new WriterPlayController(bus, () => {}, writerDocumentToJson(createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a" })));
const engagement = ctrl.mainMode.windowKinds.find((kind) => kind.id === WRITER_PLAY_WINDOW_KIND)?.engagement;
enforcePlaygroundWindowEngagementInput(engagement, "Writer play window");
if (!engagement?.input?.onSubmit) throw new Error("expected engagement input onSubmit");

console.log("[DEBUG] engagement-input-check ok");
