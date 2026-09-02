/** 🧩️ Procedural2dMutation — mirrors the 8 wired triad-leaf variants of `Procedural2dMutation`
 * (…/🧬️mutations/🦀️.rs:44-59); the other 6 variants (generation lifecycle +
 * replace-widget/replace-synapse) have no TS triad leaves. `Procedural2dMutation` carries only
 * `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with serde's default
 * EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by
 * the committed `🌱create-widget/🧪️tests/*​/🦠️mutation/🔣️.json` fixture
 * (`{"CreateWidget":{"index":2,"widget":{...}}}`). */
import type { ChangeSchema } from "./🔤change-schema/🦠️mutation/🟦️.ts";
import type { ClearWidgetLayout } from "./🧹clear-widget-layout/🦠️mutation/🟦️.ts";
import type { ConnectSynapse } from "./🔗connect-synapse/🦠️mutation/🟦️.ts";
import type { CreateWidget } from "./🌱create-widget/🦠️mutation/🟦️.ts";
import type { DeleteWidget } from "./🗑️delete-widget/🦠️mutation/🟦️.ts";
import type { DisconnectSynapse } from "./✂️disconnect-synapse/🦠️mutation/🟦️.ts";
import type { MoveWidget } from "./📍move-widget/🦠️mutation/🟦️.ts";
import type { UpdateCamera } from "./🎛set-camera/🦠️mutation/🟦️.ts";

export type Procedural2dMutation =
  | { CreateWidget: CreateWidget }
  | { DeleteWidget: DeleteWidget }
  | { ConnectSynapse: ConnectSynapse }
  | { DisconnectSynapse: DisconnectSynapse }
  | { MoveWidget: MoveWidget }
  | { ClearWidgetLayout: ClearWidgetLayout }
  | { UpdateCamera: UpdateCamera }
  | { ChangeSchema: ChangeSchema };
