import os
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts"
t=open(p,encoding="utf8").read()
a='export interface MetricV1{readonly id:string;readonly kind:MetricKindV1;readonly unit:string;readonly source:string;readonly label:{readonly en:string;readonly de:string}}'
assert a in t
t=t.replace(a,'export type MetricPlatformV1="darwin"|"linux"|"win32";\nexport interface MetricV1{readonly id:string;readonly kind:MetricKindV1;readonly unit:string;readonly scope:"origin"|"fleet";readonly source:string;readonly platforms?:readonly MetricPlatformV1[];readonly label:{readonly en:string;readonly de:string}}',1)
b='export const METRICS_CATALOGUE_V1:readonly MetricV1[]=catalogueFile.metrics as readonly MetricV1[];\n'
assert b in t
t=t.replace(b,b+'''
/** 🧭️ Whether a machine of `platform` can measure the metric `id` at all (the catalogue lists `platforms` only for a metric that some platform has no source for): a metric a platform cannot measure is `n/a` there, not a gap. An id the catalogue does not know is answered `true`.
 * @see ./🧬️schema/🔣️.json */
export function metricSupportedOnV1(id:string,platform:string):boolean{
 const platforms=METRICS_CATALOGUE_V1.find(metric=>metric.id===id)?.platforms;
 return platforms===undefined||(platforms as readonly string[]).includes(platform);
}
''',1)
tmp=p+".m7.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
