# Scale and Axis Effective Defaults Independent Audit

semio-viz-scale.sty SHA256 8C3A8022CA7E0289FE9AC84F2AB9C50167A6E3E1940DA73BF7C70A37CFC30B97
semio-viz-guide.sty SHA256 623D4F9EE92C09983F784E484978D01776EC4FF8CF4C4A55712FBE3C93A053ED

Scale source lines 86–103 define omission defaults: nice=0; clamp/reverse/round=false; padding/paddingInner/paddingOuter=0; align=0.5; base=10; exponent=1; constant=1; ticks=10; tickValues/tickFormat/scheme empty; interpolator=oklab; interval=auto; unknown=SemioVizUndefined. Bare-key defaults nice=10 and flags=true are distinct from omission. Property string storage does not establish the effective schema type; numerical and flag consumers establish it. Unknown remains typed result payload.

Axis keys lines 315–355 bind domain/domainLine, minor, grid, labels to booleans; ticks/minorTicks/breakMarks to integers; offset, label rotation aliases, radii, angles, titleGap, tick dimensions, minorSize and breakGap to floating arithmetic. Reset lines 359–399 gives ticks5, minorTicks4, offset0, tickSize and tickSizeOuter1.4, tickPadding0.8, minorSize0.7, titleGap5, labelRotate0, innerRadius0, startAngle0, endAngle360, angle90, breakGap2, breakMarks2. Outer radius is dynamic max(1,min(width,height)/2-pad), not a fixed metadata constant. Minor/grid=false; domain/domainLine/labels=true. Scale=x, orient=bottom, titleAnchor=middle, labelAlign=auto. At/from/to/center/gridLength and literal title are empty sentinels; tickValues/broken are empty lists.

These explicit shared-owner contracts must preserve direct family overrides. Axis empty geometry/sentinel fields cannot be globally coerced to numbers merely by name; inherited descriptors require owner-aware effective consumer semantics. Catalogue received the exact map before correction. This source audit makes no runtime semantic gate claim.
