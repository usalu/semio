# Smoother Summer To Winter Badge Fade

Scene: Beat6_GrenzenDerFreienLueftung.

## Problem
Sommernachmittag → Winter badge swap used morph recipe `replace` at 1.2s — felt sudden.

## Fix
1. `ventilation.natural.limits.outdoor-cold`: recipe `fade`, 2.2s, ease-in-out.
2. Scene play matches 2.2s while wash cools and air keeps circulating.
