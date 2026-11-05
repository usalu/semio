# Native UI Receiving Roster

This read-only lexical roster follows declared public UI names from UiPatch and PresenceUpdate. It is an owner checklist, not an automatic derive migration. Shared field rosters and declarations remain authoritative. Variant names can create conservative extra edges; review each actual field before editing.

Concrete records/enums must preserve exact existing serde tag/content/rename/default/skip semantics in value attributes; NativeReceive derives cannot silently bypass custom validation. RetireOwned requires genuine bounded descendants. Special owners require handwritten receive/retire adapters.

Found 97 reachable declared UI names.

## UiPatchOps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🩹️patch/🦀️.rs:30). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, PartialEq, Eq)] #[derive(Default)] #[derive(Default)]

Referenced declared UI names: UiTypedRetirementCursor.

## UiTypedRetirementCursor

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/♻️retirement/🌳️typed/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/♻️retirement/🌳️typed/🦀️.rs:10). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Default)]

Referenced declared UI names: UiValueRetirement.

## UiValueRetirement

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/♻️retirement/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/♻️retirement/🦀️.rs:35). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]

Referenced declared UI names: UiValue, UiCollectionHandle.

## UiCollectionKind

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🦀️.rs:6). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, PartialEq, Eq)]

Referenced declared UI names: none.

## UiCollectionHandle

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🏟️collection-arena/🦀️.rs:12). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, PartialEq, Eq)] #[derive(Clone, Copy, Debug, PartialEq, Eq)]

Referenced declared UI names: UiCollectionKind.

## UiList

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/📃️ui-list/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/📃️ui-list/🦀️.rs:6). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Debug, Default)]

Referenced declared UI names: UiCollectionHandle.

## UiFixedList

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔢️fixed-list/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🔢️fixed-list/🦀️.rs:5). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: 

Referenced declared UI names: none.

## UiMap

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗝️ui-map/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗝️ui-map/🦀️.rs:6). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Debug, Default)]

Referenced declared UI names: UiCollectionHandle.

## UiFixedMap

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🦀️.rs:7). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Debug, PartialEq)]

Referenced declared UI names: UiFixedList, UiText.

## UiFixedBytes

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🦀️.rs:188). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Clone, Eq, PartialEq)]

Referenced declared UI names: none.

## UiValue

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🦀️.rs:63). Kind: enum. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Debug, Default, PartialEq, Deserialize)] #[serde(untagged)]

Referenced declared UI names: UiText, UiList, UiMap.

## ActionId

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:17). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText.

## Trigger

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:48). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## ActionBinding

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:69). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Debug, Default, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: Trigger, ActionId, UiValue, UiText.

## MenuRef

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:99). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")]     #[serde(default, skip_serializing_if = "Option::is_none")]     #[serde(default, skip_serializing_if = "Option::is_none")] #[derive(Debug, Default, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: UiText, UiValue.

## UiIntent

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:124). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Debug, Default, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]     #[serde(default, skip_serializing_if = "Option::is_none")] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: SurfaceId, UiRevision, UiNodeId, UiNodeRecord, UiText, Trigger, ActionId, ActionBinding, UiValue.

## UiNodeBindings

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:146). Kind: type. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]     #[serde(default, skip_serializing_if = "Option::is_none")]     #[serde(default, skip_serializing_if = "Option::is_none")]

Referenced declared UI names: UiFixedList, ActionBinding.

## UiNodeChildren

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:27). Kind: type. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: 

Referenced declared UI names: UiFixedList, UiNodeId.

## UiNodeRecord

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:56). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: UiNodeId, UiText, Component, LayoutSpec, StyleSpec, Activity, TransitionHint, AccessibilitySpec, UiFixedList, UiNodeBindings, MenuRef, UiNodeChildren.

## UiPatch

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:205). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: SurfaceId, UiRevision, UiPatchOps.

## UiText

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔤️text/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔤️text/🦀️.rs:8). Kind: struct. Handwritten owner required.

Wire/derive attributes near declaration: #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]

Referenced declared UI names: none.

## Liveness

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/♿️accessibility/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/♿️accessibility/🦀️.rs:23). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## AccessibilitySpec

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/♿️accessibility/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/♿️accessibility/🦀️.rs:37). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, Liveness, UiText.

## Variant

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:23). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## SizeToken

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:37). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Density

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:52). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Tone

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:66). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Emphasis

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:82). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## StyleSpec

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🎨️style/🦀️.rs:95). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Variant, SizeToken, Density, Tone, Emphasis.

## Activity

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs:32). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## PeerMark

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs:49). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[serde(deny_unknown_fields)] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(semio_framework_value::RetireOwned)]

Referenced declared UI names: none.

## OwnPresence

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs:70). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[serde(deny_unknown_fields)] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(semio_framework_value::RetireOwned)]

Referenced declared UI names: Trigger.

## PresenceUpdate

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/👥️presence/🦀️.rs:102). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[serde(deny_unknown_fields)] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(semio_framework_value::RetireOwned)]

Referenced declared UI names: SurfaceId, UiNodeRecord, UiNodeId, OwnPresence, PeerMark.

## SpaceToken

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:20). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Sizing

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:79). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: SpaceToken.

## Axis

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:90). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Align

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:101). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Justify

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:114). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## GridTrack

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:128). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: SpaceToken.

## ScrollAxes

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:141). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## Anchor

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:154). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## EdgeSpace

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:172). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: SpaceToken.

## StackLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:188). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Axis, SpaceToken, EdgeSpace, Align, Justify.

## UiGridTracks

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:200). Kind: type. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiFixedList, GridTrack.

## GridLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:205). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiGridTracks, SpaceToken, EdgeSpace, Align, Justify.

## OverlayLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:230). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Anchor, EdgeSpace.

## ScrollLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:240). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: ScrollAxes, EdgeSpace, Sizing.

## AbsoluteLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:250). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Sizing.

## LeafLayout

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:259). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Sizing.

## LayoutSpec

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs:270). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(tag = "kind", rename_all = "camelCase")] #[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase")]

Referenced declared UI names: LeafLayout, StackLayout, GridLayout, OverlayLayout, ScrollLayout, AbsoluteLayout.

## UiPublicationRevision

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs:6). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]

Referenced declared UI names: none.

## Label

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs:59). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)] #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)]

Referenced declared UI names: UiText.

## TransitionHint

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🔖️labels/🦀️.rs:94). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## SurfaceKind

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs:81). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)] #[value(crate = "::semio_framework_value")]

Referenced declared UI names: SurfaceProps.

## SurfaceDoc

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs:137). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[value(rename = "diff-view")]     #[serde(rename = "event-feed")]     #[value(rename = "event-feed")] #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[value(crate = "::semio_framework_value")]

Referenced declared UI names: UiFixedBytes.

## SurfaceProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs:151). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[value(rename = "event-feed")] #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[value(crate = "::semio_framework_value")] #[derive(Debug, Default, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: SurfaceKind, UiText, SurfaceDoc, UiFixedList, UiNodeBindings.

## TreeProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:13). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: TreePresentation, UiText.

## TreePresentation

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:28). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## TreeWindowRowExtent

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:45). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## TreeWindow

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:60). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: TreeWindowRowExtent.

## TreeSectionProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:93). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, UiNodeId, TreeWindow.

## TreeItemProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🌳️tree-props/🦀️.rs:122). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[value(default, skip_serializing_if = "Option::is_none")]     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: Label, UiText, UiFixedMap, TreeWindow, ContainerRole, Component, UiNodeId, UiFixedList, RowAction, RowTarget.

## ContainerRole

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:10). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## InputKind

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:33). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## RowActionPlacement

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:47). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## DropOverlaySpec

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:59). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, UiText.

## SelectItem

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:71). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText, Label.

## KeyValueEntry

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:80). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, UiText.

## RowAction

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:92). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: ButtonProps, UiText, Label, RowTarget, RowActionPlacement.

## RowTarget

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎛️container-roles-and-row-actions/🦀️.rs:149). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Copy, Debug, PartialEq, Eq)] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: UiText, UiMap.

## ContainerProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:13). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: ContainerRole, Label, UiText, DropOverlaySpec.

## TextProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:46). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, UiFixedMap, UiText.

## ButtonProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:80). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText, Label.

## SeparatorProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:96). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[value(crate = "::semio_framework_value")]

Referenced declared UI names: none.

## InputProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:103). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[value(crate = "::semio_framework_value")] #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: InputKind, UiText, Label, SliderProps, UiFixedList, UiNumberLimits, UiPublicationRevision.

## SelectProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:179). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText, UiFixedList, SelectItem, Label, SelectAppearance.

## SelectAppearance

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:196). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "SelectAppearance::is_menu")]     #[value(default, skip_serializing_if = "SelectAppearance::is_menu")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## ToggleProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:214). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: ToggleAppearance, UiText, Label.

## ToggleAppearance

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:229). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## KeyValueListProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:245). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiFixedList, KeyValueEntry.

## RingProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:253). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText.

## IconSelectProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:262). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText.

## ProgressProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🎨️basic-widget-props/🦀️.rs:276). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label.

## UiNumberScale

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:11). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## SliderAppearance

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:29). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: none.

## UiNumberBound

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:53). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label.

## UiNumberLimits

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:68). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiNumberBound.

## SliderProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:119). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText, UiFixedList, SliderAppearance, UiNumberScale, UiNumberLimits.

## NumberStepperProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🔢️number-scale-and-slider-props/🦀️.rs:170). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiFixedList, UiText, UiNumberLimits.

## TableProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:16). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: Label, UiFixedList, TreeWindow.

## TableRowProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:51). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[value(default, skip_serializing_if = "Option::is_none")]     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: UiFixedList, UiText, RowAction, RowTarget.

## ImageProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:72). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration:     #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]     #[serde(default, skip_serializing_if = "Option::is_none")] #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)] #[serde(rename_all = "camelCase")] #[value(crate = "::semio_framework_value", rename_all = "camelCase")]

Referenced declared UI names: UiText, Label.

## ExtensionProps

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:87). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[value(crate = "::semio_framework_value", rename_all = "camelCase")]     #[serde(default, skip_serializing_if = "Option::is_none")]     #[value(default, skip_serializing_if = "Option::is_none")] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]

Referenced declared UI names: UiText, UiValue, LayoutSpec, StyleSpec, AccessibilitySpec, ActionBinding, MenuRef, Activity, SurfaceProps, ActionId, Trigger, UiIntent.

## Component

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🗃️table-image-extension-and-component/🦀️.rs:113). Kind: enum. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(rename_all = "camelCase")] #[derive(Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "type", rename_all = "camelCase")]

Referenced declared UI names: ContainerProps, TextProps, ButtonProps, SeparatorProps, InputProps, SelectProps, ToggleProps, KeyValueListProps, SliderProps, NumberStepperProps, RingProps, IconSelectProps, ProgressProps, TreeProps, TreeSectionProps, TreeItemProps, ImageProps, SurfaceProps, ExtensionProps, TableProps, TableRowProps.

## SurfaceId

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs:10). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)] #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)]

Referenced declared UI names: UiText.

## UiNodeId

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs:45). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)] #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)]

Referenced declared UI names: none.

## UiRevision

Owner: [🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs](C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🆔️identity/🦀️.rs:52). Kind: struct. Concrete typed projection candidate; inspect defaults and validation.

Wire/derive attributes near declaration: #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)] #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)] #[serde(transparent)] #[value(crate = "::semio_framework_value", transparent)]

Referenced declared UI names: none.

## Existing Shared Field Rosters

Presentation and interaction field-roster modules already enumerate each concrete payload for typed retirement/copy/compare. Reuse those owners for exact fixed-field ports. Do not create an alternative UI DTO or discard fields.

## Verification

Bounded UiText neutral UTF8 corpus: Ajv + SQLite 1 pass, 19 assertions. Fixed collections neutral corpus: Ajv + SQLite 1 pass, 13 assertions. Native finite-wallet/heap/cancel laws authored; real native compile is pending. Full TurnResult/UI cascade and process-lifetime arena registration are incomplete.
