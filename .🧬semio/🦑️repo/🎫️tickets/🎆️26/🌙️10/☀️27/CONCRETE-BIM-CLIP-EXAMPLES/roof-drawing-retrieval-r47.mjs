/** 🔎 Reproduce bounded public UVaDOC metadata connectivity attempts. */
const urls = [
  'https://uvadoc.uva.es/handle/10324/66393',
  'https://uvadoc.uva.es/oai/request?verb=GetRecord&metadataPrefix=mets&identifier=oai%3Auvadoc.uva.es%3A10324%2F66393',
  'https://uvadoc.uva.es/metadata/handle/10324/66393/mets.xml',
  'https://uvadoc.uva.es/bitstream/handle/10324/66393/SAVOYE%20PLANTA%2B2.pdf',
  'https://uvadoc.uva.es/bitstream/10324/66393/4/SAVOYE%20PLANTA%2B2.pdf',
];
for (const url of urls) {
  try {
    const response = await fetch(url, { signal: AbortSignal.timeout(12000) });
    console.log('[DEBUG] ', JSON.stringify({url, status: response.status, type: response.headers.get('content-type')}));
    await response.body?.cancel();
  } catch (error) {
    console.log('[DEBUG] ', JSON.stringify({url, error: String(error), cause: String(error.cause)}));
  }
}
