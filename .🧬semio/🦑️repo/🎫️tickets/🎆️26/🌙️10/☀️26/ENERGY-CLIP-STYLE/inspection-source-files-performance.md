# Browser Source Files Performance

Actual source-only profiling; no UI frames or production edits.

[
  {
    "function": "_call",
    "seconds": 15.410142799999448,
    "count": null
  },
  {
    "function": "_membership",
    "seconds": 4.194966299999578,
    "count": 650
  },
  {
    "function": "_identity",
    "seconds": 0.9982301999998526,
    "count": null
  },
  {
    "function": "_membership",
    "seconds": 7.0256915999998455,
    "count": 650
  },
  {
    "phase": "cold",
    "seconds": 21.10520919999999,
    "sources": 2520,
    "members": 650,
    "missing_count": 0,
    "missing_sample": []
  },
  {
    "function": "_membership",
    "seconds": 4.966330399999606,
    "count": 650
  },
  {
    "function": "_identity",
    "seconds": 3.180833299999904,
    "count": null
  },
  {
    "function": "_membership",
    "seconds": 6.244736200000261,
    "count": 650
  },
  {
    "phase": "warm",
    "seconds": 8.149294300000292,
    "sources": 2520,
    "members": 650,
    "missing_count": 0,
    "missing_sample": []
  }
]


## Post-Fix Independent Rerun

Exact retained profiler rerun, with no UI frames or production edits: cold 21.105 s, warm 8.149 s. Warm discovery _call count is zero; only one _call occurred in the complete cold/warm run (cold 15.410 s). Both phases return 2,520 sources and cover all 650 manual candidates, missing_count=0. The two previously absent Canvas storybook-interaction TS/TSX candidates are therefore now included; closure grew by exactly two paths.

Before fix: cold 22.067 s, warm 28.319 s, 2,518 sources, two missing candidates, warm discovery 18.726 s. The new warm call is 3.48 times faster (71.22 percent lower elapsed time), despite remaining real membership and content-hash work. New warm membership 4.966 s and _identity 3.181 s account for almost all 8.149 s. Extra diagnostic membership sampling is excluded from timed phases.

Read-only source review confirms unchanged warm admission requires both exact membership equality and full _identity(_SOURCES). Discovery now unions conservative candidate paths before hashing, explicitly rejects missing roots and rechecks every candidate file exists before committing closure. Thus the speedup removes false repeated discovery without dropping content, membership, missing-root or disappeared-file guards. Root portable lifecycle tests separately cover changing candidate bytes, additions and removals; this integration run does not mutate production inputs to repeat those tests.

The original detailed pre-fix profile was overwritten by the exact retained profiler input; causal baseline timings and missing path identities are preserved here and in the preceding task audit messages. Previously missing paths: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/🟦️.ts and its nested 🧪️tests/🟦️.tsx.

A subsequent existing concurrent-discovery oracle caught a real race: a candidate added while the compiler enumerated inputs could enter the conservative hash set without its imports/assets entering the discovered graph. The final implementation reaches a bounded stable fixed point: cold discovery learns roots, then verifies candidate membership and all known byte fingerprints before/after compilation, including any newly discovered graph inputs. Warm reuse remains hash/membership-guarded and does no compiler call. The original racing late-candidate case plus six-step cache lifecycle and four persistent-asset tests pass together (6tests,1.578s). Cold call-count fixtures now explicitly require two discovery passes; no time/mtime heuristic is used.
