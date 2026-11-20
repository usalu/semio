# Separate Winter And Summer Cards

Scene: `Beat1_Lueftungsbedarf` jahreszeit beat.

## Problem
Fixed 1.4-tall frames at y=1.15 and y=−0.15 overlapped by ~0.1.

## Fix
Cards size to content, then `arrange(DOWN, buff=0.32)` as a stack on the right.
