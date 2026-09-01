/** 🧩️ Procedural2dMutation — mirrors the 8 wired triad-leaf variants of `Procedural2dMutation`
 * (…/🧬️mutations/🦀️component.rs:44-59); the other 6 variants (generation lifecycle +
 * replace-widget/replace-synapse) have no TS triad leaves. `Procedural2dMutation` carries only
 * `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with serde's default
 * EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by
 * the committed `🌱create-widget/🧪️tests/*​/🦠️mutation/🔣️component.json` fixture
 * (`{"CreateWidget":{"index":2,"widget":{...}}}`). */
import type { ChangeSchema } from "./🔤change-schema/🦠️mutation/🟦️component.ts";
import type { ClearWidgetLayout } from "./🧹clear-widget-layout/🦠️mutation/🟦️component.ts";
import type { ConnectSynapse } from "./🔗connect-synapse/🦠️mutation/🟦️component.ts";
import type { CreateWidget } from "./🌱create-widget/🦠️mutation/🟦️component.ts";
import type { DeleteWidget } from "./🗑️delete-widget/🦠️mutation/🟦️component.ts";
import type { DisconnectSynapse } from "./✂️disconnect-synapse/🦠️mutation/🟦️component.ts";
import type { MoveWidget } from "./📍move-widget/🦠️mutation/🟦️component.ts";
import type { UpdateCamera } from "./🎛set-camera/🦠️mutation/🟦️component.ts";

export type Procedural2dMutation =
  | { CreateWidget: CreateWidget }
  | { DeleteWidget: DeleteWidget }
  | { ConnectSynapse: ConnectSynapse }
  | { DisconnectSynapse: DisconnectSynapse }
  | { MoveWidget: MoveWidget }
  | { ClearWidgetLayout: ClearWidgetLayout }
  | { UpdateCamera: UpdateCamera }
  | { ChangeSchema: ChangeSchema };
