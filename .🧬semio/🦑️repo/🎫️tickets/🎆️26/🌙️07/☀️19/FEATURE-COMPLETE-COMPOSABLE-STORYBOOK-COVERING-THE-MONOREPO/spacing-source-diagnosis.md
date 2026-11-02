# Spacing Source Diagnosis

Read-only scout handoff: `result:spacing-diagnosis#1`, accepted alongside lead browser baselines in `🗑️generated/visual-before`. These are multiple local defects, not a reason to enlarge the global spacing scale. All paths below are relative to `🧰️framework/🔨️modules/🖱️ui`.

## Badge

`🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx` references undefined `px-half` and `gap-half`. Browser horizontal padding is zero. Use existing semantic `px-double` (6.4px compact) and `gap-single` (3.2px); preserve sensible vertical clearance.

## Label

`🧱️elements/🏷️Label/🟦️.tsx` measures allocated wrapper scroll widths as intrinsic widths, prematurely stacking the 280px story. Label wrapper becomes280px, control moves below and retains102.4px inline-track indentation. Label wrapper's normal line box also differs from the inline label height. Fix intrinsic-fit measurement, intentional narrow-width stacking without indentation, and vertical alignment. Shared property geometry helpers are in `🧱️elements/🌳️Tree/📏️row-alignment/🟦️.tsx`; connector and property-layout definitions need separate owners.

## ActionGroup

`🧱️elements/⚡️ActionGroup/🟦️.tsx` combines a16px bordered shell with inherited22.4px item minimum height and3.2px padding. A more-specific classless-SVG shared rule defeats its generic tiny-icon rule, leaving16px icons. Normalize shell/item/icon geometry; do not clip overflow to hide it. Shared chrome presentation lives in `🔨️modules/🎛️chrome-control-presentation/🟦️.ts`.

## Avatars

PresenceBar's `-space-x-2` overlaps16px avatars by8px. Its2px border leaves12px for11.2px text with16px line-height. TableAvatar's16px border box likewise lacks text clearance. Use a shared readable avatar dimension (existing24px `size-workbench` is suitable), positive semantic roster gap, consistent overflow chip sizing, and preserved focus/activity indicators. Sources: `🧱️elements/👥️PresenceBar/🟦️.tsx` and `🧱️elements/📻️TableAvatar/🟦️.tsx`.

## Shared Class Composition

`🔨️modules/🏷️class-name-composition/🟦️.ts` classifies every arbitrary `text-[…]` utility as font size. Selected avatar color `text-[color:var(--active-foreground)]` therefore erases `text-xs`, producing16px/24px typography instead of11.2px/16px. Correct color-versus-size classification, including semantic `text-tiny`/`text-2xs`, rather than re-adding classes in a component.

## Authority and Validation

Canonical compact spacing is3.2px, small control16px, medium22.4px; XS text11.2px/16px and SM12.8px/19.2px. Styling source is `🎨️styling/🔣️.json`, geometry bindings `🎨️styling/🌓️theme/📐️geometry/🔣️.json`, authored utilities `🎨️styling/🖌️ui/🎨️.css`. Do not hand-edit generated CSS/tokens.

Extend existing class-composition conflict feature/tests and TableAvatar component tests. Add language-neutral fixtures plus independent third-party or browser geometry oracles. Lead owns browser acceptance and static server refresh.
