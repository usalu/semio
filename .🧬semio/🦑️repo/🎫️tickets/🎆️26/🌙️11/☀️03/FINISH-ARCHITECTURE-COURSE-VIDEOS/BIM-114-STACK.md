# BIM 114 Native Stack

A bounded read-only py-spy sample of current source 114 native PID 48976 found the main thread actively performing exact Fraction clipping in positive-face / clip-stock / reference-material-parts / column-partition / solid-geometry, called from the beam list in building-geometry source line 2776. This is before the first authored animation and is not evidence of an idle hang.

The same building-geometry call evaluates the slab list earlier at line 2764. The process environment specifies the ticket generated/full-slab-v2 cache. The stack sample confirms progress past that list and the configured cache path, but cannot by itself establish individual cache hits. No process, source or cached geometry was changed by this observation.
