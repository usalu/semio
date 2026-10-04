# Chart Current Norm Property And Intermediate Presence Map

Read-only actual schema/model audit. Exact definitions, enums, required fields, number constraints and property roles are retained without inferred defaults in `📥️inputs/current-chart-norm-property-census.json`, copied from actual Print `🧬️schema/🔣️.json` Chart definitions. No execution or provider change.

| Entity | Required fields | Optional fields and child roles |
| --- | --- | --- |
| Chart | width,height,language,layers | margin,theme,tables,scales,coordinate,guides,title,annotations,presets |
| Margin | top,right,bottom,left | none |
| Theme | none | name,appearance(light/dark),palette ordered strings |
| Localized text | en,de | none; preserve both independently |
| Table | name,columns,rows | columns ordered strings; rows ordered maps of string/number/bool/null cells |
| Scale | name,kind,domain,range | options; domain/range ordered heterogeneous number/string arrays |
| Transform | kind | options open map scalar string/number/bool or ordered array of those scalars |
| Layer | mark | data,transform ordered steps,layout,encodings,options |
| Layout | algorithm | options open string/number/bool map |
| Encoding | none | column,scale,value(string/number/bool); empty encoding is allowed |
| Coordinate | kind | options open map scalar string/number/bool or array of those scalars |
| Guide | kind | scale,orient,title,ticks,tickValues,tickFormat,tickSize,minor,grid,offset,options |
| Annotation | kind,x,y | x2,y2,width,height,radius,text,options |
| Preset | kind | data,options |

Every named object is closed against extra properties, except explicitly open row/option maps. Encoding channel keys are exactly the20 `VIZ_CHANNELS`/schema property names. Closed enum lists for15 scale kinds,17 transforms,18 marks,24 layouts,7 coordinates,3 guide kinds/4 orientations,7 annotation kinds and language en/de are preserved verbatim in machine input. Scale options are a closed18-field entity: nice,clamp,padding,paddingInner,paddingOuter,align,round,base,exponent,constant,ticks,tickValues,tickFormat,interpolator,scheme,reverse,interval,unknown.

Numerical norm: width/height strictly positive; guide ticks integer≥1; scale nice bool or strictly-positive number; padding/paddingOuter≥0; paddingInner/align in[0,1]; base>0 and!=1; constant/ticks>0; scheme nonempty; table name/preset kind nonempty. Other numerical fields admit schema numbers without invented integer bounds. Null is only an actual row-cell alternative and scale-option unknown; absent is distinct from explicit null. Booleans false, zero, empty string, empty map, empty array and missing optional entities remain distinct. JSON defaults guide scale=x and tickSize=1.4 are annotations, not authored values to materialize during persistence.

Array cardinality has no minItems/uniqueness restrictions in these definitions. Table columns may be empty/repeated; row maps are not constrained to the declared column list and can contain missing/extra cell keys. Preserve all row cells rather than projecting only declared columns. Transform arrays can mix string/number/bool; they cannot contain nested arrays/objects/null. Domain/range/tickValues likewise preserve mixed number/string order. Optional lists/maps need explicit presence so missing versus present empty does not collapse.

Current Source `VizCoordinateOptions` is narrower than JSON: it names geometry numeric fields, axes string array, projection string and transpose/normalized flags; authoritative ChartSpecification JSON allows arbitrary typed scalar/array coordinate option keys. This is an actual norm/facet mismatch; do not silently drop schema-admitted options or infer unknown keys are invalid from Source's interface alone. Source guide ticks is number while JSON requires integer≥1; JSON imposes additional bounds elsewhere too.

The Rust owner is currently any DslValue, and generic ChangeChartValue accepts arbitrary path and optional value (`🧬️schema/🧬️mutations/🦀️.rs:6–10`). Its default has width80,height40,layers[] but missing mandatory language. Preserve it as an explicitly incomplete intermediate owner with language presence absent, while the valid-publication ChartSpecification continues to require en/de. The coherent extension is a separate closed authored-intermediate chart persistence contract with presence for declared required fields where partial editing is admitted, and a separate validator for completed ChartSpecification. It must still use chart-specific entities, roles and enum alternatives; it must not widen unknown/nested arbitrary values into a whole-chart Value/JSON escape hatch. Other incomplete fields admitted by generic mutation require explicit future contract decisions rather than unbounded generic preservation or invented defaults.

Numbers retain exact Native numeric alternatives/words where the authored intermediate contract admits them; valid JSON publication and inference have their distinct finite-number restrictions. Do not make successful SQL equality depend solely on current chart_values_equal, which compares number as_f64 and object fields by key lookup: independent SQL typed/raw-word and ordered-row/array demands are needed for persistence.
