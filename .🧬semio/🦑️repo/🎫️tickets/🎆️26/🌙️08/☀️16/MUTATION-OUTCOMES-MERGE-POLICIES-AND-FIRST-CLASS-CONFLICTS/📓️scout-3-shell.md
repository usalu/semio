# Lane 2-D: Shell UI + i18n Surfaces Mapping

**Date:** 2026-08-16 | **Scout:** Claude Haiku 4.5

---

## 1. Settings Surface Ownership

**Component:** ChromePanels
**Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ChromePanels/🟦️component.tsx` (1283 lines)

**Role:** Owns the **Settings panel** with three tabs: General/Theme/Keybindings/DefaultApps. The "merge policy" toggle row will go in **General tab** (section `framework.settings.general`).

**Settings Row Pattern** (lines 372–424):
```tsx
{
  id: "framework.settings.layout",
  label: shellLabel("ui.settings.tab.layout"),
  control: (
    <Select value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
      <SelectTrigger id="framework.settings.layout" className="h-small w-32" size="sm">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="desktop">{shellLabel("settings.layout.desktop")}</SelectItem>
        <SelectItem value="tablet">{shellLabel("settings.layout.tablet")}</SelectItem>
      </SelectContent>
    </Select>
  ),
}
```

**New row placeholder:**
```tsx
{ id: "framework.settings.merge-policy", label: shellLabel("ui.settings.merge-policy"), control: <Select/> }
```

---

## 2. DocumentChanged Frame End-to-End

**Seam:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts`

- **Encoder** (lines 2115–2118): `writeVecBytes(out, frame.DocumentChanged.envelopes); writeStr(out, frame.DocumentChanged.origin)`
- **Decoder** (lines 2241–2244): `return { DocumentChanged: { envelopes, origin } }`
- **Frame Tag:** `APP_FRAME_TAGS.DocumentChanged = 6`
- **Unsolicited:** No `in_reply_to` — flows directly from backbone sync to subscribers

**Routing for MergeReport/Conflicts:** Add new cases alongside DocumentChanged in the frame switch (lines 2115+), then dispatch via same channel that feeds EventFeedHost/ChromePanels observers.

---

## 3. Config Mutation Triads

**Base Path:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/`

**Existing Triads:** 
- `📌️set-default-app/`
- `🧹clear-default-app/`

**Triad Layout (reference: set-default-app):**
```
📌️set-default-app/
├── 🦠️mutation/🦀️component.rs
│   ├ #[derive(...)] pub struct SetDefaultApp { dialect, role, app }
│   ├ impl MutationKind { const SEMANTICS, fn diff(), fn inverse(), fn label(), fn target() }
│   └ #[cfg(test)] mod tests
├── 🔺️diff/🦀️component.rs
│   └ pub fn diff(payload, base) -> filtered + appended OpeningPreferences
└── ↩️inverse/🦀️component.rs
    └ pub fn inverse(payload, base) -> Vec<...> (reads BASE, not diff result)
```

**For 🛡️change-merge-policy:** Create three files with same structure, wrapping `SetMergePolicy { dialect, policy_kind }` payload.

---

## 4. i18n Schema & Bundles

**Schema Type:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx`

**Bundles (de + en):** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` (de: lines 2230–2748, en: lines 2752–3482)

**Label Pair Pattern** (lines 2641–2714):
```tsx
settings: {
  layout: {
    desktop: { label: { normal: "Desktop-Layout", beginner: "Verwendet das Standard-Layout..." } },
    tablet: { label: { normal: "Tablet-Layout", beginner: "..." } },
  },
  driver: { ... },
  keybindings: { capture: {...}, reset: {...}, conflict: {...}, pressKeys: {...} }
}
```

**Placement for new keys:**
- `ui.mutation.*` → add new subsection under `ui:` after `settings` (alphabetically)
- `ui.conflict.*` → add new subsection under `ui:` after `mutation`
- Each entry: `{ label: { normal: "...", beginner: "..." } }`

---

## 5. HistoryTable Badge Column

**Path:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️HistoryTable/🟦️component.tsx` (280 lines)

**Row Model** (lines 24–33):
```tsx
export interface HistoryColumn {
  readonly checkpointId: string;
  readonly timestamp: string;
  readonly labels: readonly string[];
  readonly authors: readonly HistoryColumnAuthor[];
  readonly parentCheckpointId?: string;
  readonly description?: string;
  readonly lane: number;
  readonly alternativeIds: readonly string[];
}
```

**Add Badge:**
- Extend `HistoryColumn` with optional `mutationKind?: string` or `badges?: { kind: string; tone: "info"|"warning"|"error" }[]`
- Insert new grid column in render layout (line 169): add third column position
- New render block after description (line 197): map badge array → chip elements (similar to `HistoryRowLabels` at lines 129–142)

---

## Summary: Key Seams for Lane 2-D

| Concern | File | Line(s) | Nature |
|---------|------|---------|--------|
| **Settings toggle** | ChromePanels | 329–425 | Tree item with Select control |
| **Frame rx/tx** | OS component | 2115–2244 | Encoder/decoder switch cases |
| **Mutation triad** | config/mutations | — | Three-file pattern (mut/diff/inverse) |
| **i18n schema** | I18n/component.tsx | 93–558 | UiTranslationSchema type def |
| **i18n bundles** | react/index.tsx | 2230–3482 | de/en nested { label: { normal, beginner } } |
| **History rows** | HistoryTable | 24–202 | HistoryColumn + grid render |

