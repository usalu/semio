# XLSX persisted mutation wire

The Base, Strict, and Transitional editors persist the same Base `XlsxMutation`. The previous encoder wrote a private format byte and variant tag, while the original bounded receiver opened the same bytes as a complete Semio Pack document. A genuine archive reload therefore failed at Pack verification before reaching the mutation.

The cold mutation encoder and decoder now use the existing Pack value bridge. Its top-level field 1 contains the mutation value defined by the Base schema, including the internal `mutation` discriminant. The cold decoder and the bounded receiver both normalize whole numeric values before binding the schema. The authored protocol describes the Pack envelope and delegates its segments and checksums to the Pack format.

The existing archive regression exercises the original receive path. The neutral Insert Cell witness now also round-trips through the mutation wire and rejects a corrupt Pack header checksum. The decoded mutation must produce the committed workbook snapshot and export a workbook in which Calamine reads 42 at B2. The existing protocol walk checks the encoded bytes for every demo mutation variant.

The native verification run is coordinated with the archive replay changes to share one compiler invocation. See [native verification](native-verification.md).

The standalone Strict conformance mutation now also emits the complete Pack document its original receiver expects. The VML fixture asserts the framed bytes decode directly to the declared Strict mutation value, then checks type recovery, invalid magic, inverse behavior, and XML output against quick-xml. Transitional conformance supplies no separate binary mutation encoder or receiver.
