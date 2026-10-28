# Canonical Cursor And Interaction Audit

Read-only source audit on 2026-10-26. All source paths below are relative to 🧰️framework/🔨️modules. Native visual review pending.

## Dark Appearance Paint

Authority: 🖱️ui/🎨️styling/🔣️.json appearances.dark.chrome and 🖱️ui/🔨️modules/🖱️interaction-presentation/🟦️.ts. Normal border and element are gray #7b827d. Hover fill is dark-4-7 #606966, emphasized content/border is light #f7f3e3. Active/selected fill and border are primary #ff344f, foreground dark #001117. Active-hover is 80% primary plus 20% light. The helper 🖱️ui/🎨️styling/🌓️theme/🌈️color/🟦️.ts blendTokenHex weights the first token by ratioA, mixes each stored RGB channel directly and rounds; it documents this as the headless sRGB approximation.

Generic hover is cream emphasis, not a primary glow. Active selection owns primary. Disabled chrome controls have pointer-events none and opacity 0.5. There is no canonical generic click ripple.

## Cursor Geometry And Assets

Authority: 🖱️ui/🎨️styling/🖌️ui/🎨️.css lines 617–640 overrides default cursor assets for dark appearance. CSS uses 🖼️assets/👆️cursor/🖱️default/🌙️dark.svg, 👆️pointer/🖋️dark-source.svg and ☑️selectable/🌙️dark.svg. The light SVG variants do not match the dark appearance.

All are the same two-part arrow geometry with viewBox 0 0 14.666391 23.899563 (selectable height differs by 0.000002), hotspot 0,0, stroke width 0.25 and miter limit 5. Read their two native SVG paths and group translation directly. Default: both shapes cream fill with #000117 stroke. Pointer: main shaft primary fill, secondary diamond cream fill, #001117 stroke. Selectable: both cream fill with primary stroke. Fullscreen Toggle uses cursor-selectable; title mode-dock tabs use cursor-pointer. There is no generic pressed cursor token. Grabbing is a separate drag state and uses the light grabbing SVG with teal #00a69d fill; it must not be substituted for a click state.

## Parent And Tree Emphasis

Window silhouette CSS at lines 7217–7222 emphasizes the direct child normal-kind silhouette of each hovered stack with border-emphasized-color, stroke transition 120 ms ease. It explicitly excludes focus-within and non-normal kinds. Descendant targets cause their actual ancestors to match hover; sibling windows must remain unchanged. Border overlays are pointer-events none. Primary active silhouettes retain active color.

There is no universal canonical all-element parent resolver. Navbar/Footer are sibling base-level bands, while canvas contains windows and controls. The video interaction hierarchy must represent these explicit parent chains and extend emphasis to all ancestors as requested, without interpreting unrelated siblings as ancestors or flooding every descendant. Transparent U-gaps are not painted content and should use silhouette hit geometry.

Tree authority: 🖱️ui/🧱️elements/🌳️Tree/🧭️hover-path/🟦️.ts and 🧷️path-marking/🟦️.ts. Resolve the closest known row within root; otherwise use nearest branch owner row. Branch ownership checks previous or next sibling, including collapsible-content around section rows. Mark hovered row and all owner rows up to but excluding root, with intervening branches. Terminal child branch is also marked. Pointer leave clears hover path; selection path is separately synchronized and persists.

Tree visual authority: 🖱️ui/🌐️globals/🎨️.css lines 97–121. Ancestor guide lines, elbows and stems become emphasized light. Guide/stem width and elbow height become 1.5 hairline. Row labels, icons, actions and gutter icons on either hover or selection path become emphasized light. This does not prescribe primary-filled ancestor rows.

## Fullscreen And Hit Regions

Navbar fullscreen authority: 🖱️ui/🎯️targets/⚛️react/🖥️fullscreen/🟦️.tsx NavbarFullscreenToggle. It uses Toggle pressed=isFullscreen, off icon Maximize2Icon, on icon Minimize2Icon, localized enter/exit text, and state synchronized from fullscreenchange/webkitfullscreenchange. Commit on click unless cancelled or disabled; keyboard supports Enter and Space release through Toggle.

Icon assets: 🖼️assets/🔣️icons/🔎️viewing/🔲️maximize-2.svg and 🔳️minimize-2.svg. Both viewBox 0 0 24 24, fill none, currentColor stroke width 2, round caps/joins. Maximize paths: M15 3h6v6; m21 3-7 7; m3 21 7-7; M9 21H3v-6. Minimize paths: m14 10 7-7; M20 10h-6V4; m3 21 7-7; M4 14h6v6.

Control cell is the hit area, not the icon path. Canonical chrome uses medium-height controls, padding single, inline gap single; default SVG size-small. WindowChrome enlarge/close cap controls are separate buttons with icon and tiny text; title chip/control cells own glass while the U-gap stays transparent. WindowChrome border overlay never catches hits.

## Normal Control Text And Fill Confirmation

chromeControlItemBaseClass in chrome-control-presentation is text-element plus bg-transparent. CSS text-element resolves hover-ink when supplied, otherwise border-element-color (dark theme gray). The hover utility sets hover-ink to emphasized. modeDockTabClassName inherits that same transparent/gray base; WindowChrome title chip is borderless/transparent over its glass cap cell. Generic window bodies instead inherit text-foreground (light). Do not paint all normal controls as opaque dark rectangles; transparency should preserve their parent level glass fill.
