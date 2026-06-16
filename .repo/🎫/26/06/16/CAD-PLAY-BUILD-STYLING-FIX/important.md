Follow-up to spacing fix: production also dropped glass, radius, toolbar, z-index, and typography tokens from @layer base :root.

Moved chrome tokens into @theme inline (same mechanism as --ui-spacing fix).

Verified on vite preview with concrete-forest fixture:
- panel backdrop: blur(40px) saturate(1.45)
- toolbar anchor bottom uses --toolbar-footer-offset
- window-measures borderRadius: 0px
- toolbar glass blur active
