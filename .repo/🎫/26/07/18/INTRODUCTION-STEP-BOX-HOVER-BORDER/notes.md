# Notes

## Change

Introduction step boxes (`[data-slot="introduction-info-box"]`, shared recipe with `[data-slot="dialog-box"]`) no longer bake a permanent emphasized border via bare Tailwind `border` + `text-foreground` (`currentColor`).

Border is CSS-owned: `--border-normal-color` at rest, `--border-emphasized-color` on `:hover` / `:focus-within`.
