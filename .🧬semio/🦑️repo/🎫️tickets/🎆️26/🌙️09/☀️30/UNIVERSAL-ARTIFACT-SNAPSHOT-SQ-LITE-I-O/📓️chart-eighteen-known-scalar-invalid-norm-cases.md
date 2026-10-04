# Chart Eighteen Known Scalar Invalid Norm Cases

Read-only actual normative property inspection; no validator/test invocation. Exact cases are retained in `📥️inputs/chart-known-scalar-admission-invalid-cases.json`, each applying one field edit to the existing rich complete case, with expectedValid=false. Paths/rules come from actual Print schema ChartGuide and ChartScaleOptions, not inferred rendering policies.

| Edited path | Invalid value | Actual false norm |
| --- | --- | --- |
| guides[0].ticks | 0 | integer minimum1 |
| guides[0].ticks | 1.5 | integer type |
| guides[0].orient | diagonal | top/right/bottom/left enum |
| scales[0].options.nice | 0 | boolean or number>0 |
| scales[0].options.nice | -1 | same |
| scales[0].options.padding | -1 | minimum0 |
| scales[0].options.paddingInner | -0.1 | minimum0 |
| scales[0].options.paddingInner | 1.1 | maximum1 |
| scales[0].options.paddingOuter | -1 | minimum0 |
| scales[0].options.align | -0.1 | minimum0 |
| scales[0].options.align | 1.1 | maximum1 |
| scales[0].options.base | 0 | strictly positive |
| scales[0].options.base | 1 | not const1 |
| scales[0].options.constant | 0 | strictly positive |
| scales[0].options.ticks | 0 | strictly positive |
| scales[0].options.interpolator | linear | number/round/rgb/lab/hcl/oklab enum |
| scales[0].options.interval | hour | auto/day/week/month/year enum |
| scales[0].options.scheme | empty string | minLength1 |

The actual current provider's type-only role checks accept these values unless separately patched; Root should establish its genuine independent AJV false pins and actual projection/reconstruction failures before Physical's bounds closure. No passing or failing runtime result is asserted here.
