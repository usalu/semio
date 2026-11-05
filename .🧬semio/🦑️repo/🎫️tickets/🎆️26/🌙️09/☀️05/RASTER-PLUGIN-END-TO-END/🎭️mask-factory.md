# Mask and Layer Bake Execution Profiles

The current route census has 28 general commands, two dedicated layer bake commands, selection, mask and download: 33 command/proof rows. The old public selection setter is removed.

Mask input permits 139,264 bytes: three escaped 4,096-byte business/address fields plus 65,536 bytes of fixed framing. Its output ceiling is 268,541,952 bytes: maximum 16,777,216 pixels × four channels × four encoded bytes, four escaped identifier occurrences and 8,192 fixed bytes. This profile belongs only to the mask factory.

Flatten and merge use their dedicated resumable layer bake factory: raw 65,536 bytes, 4,096-byte chunks, 1,024 work units per turn, output 372,842,496 bytes, 7,500 microseconds, depth 64 and one item per turn. Both command schemas now carry the same execution metadata as the neutral fixture. The output ceiling includes the maximum RGBA array, up to 4,096 forward deletion identifiers, 64 asset removal identifiers and fixed new-layer metadata. Forward deletes carry identifiers; this is not a bound for serializing every original layer's complete metadata. The compositor retains its separate 1,024 selected-node limit; global discovery/reference census remains 4,096.

The ticket's publication-envelope-oracle.ts executed through repo and passed 147 encoding/envelope vectors and both command schema/profile matches. It independently uses ECMAScript JSON encoding plus Ajv validation; it does not execute Rust publication. A Rust Serde JSON regression was authored for all 32 control characters, quotes, backslashes, multibyte identifiers and sample values 0/128/255. That regression has not run.

Execution contracts bound producer transport; they do not establish sufficient original retained memory, total fuel, mounted admission, compiler success or end-user behavior. Those remain integration obligations. The latest JS attempt terminated during graph planning before any tests ran; see 🧪️unrestricted-verification.md.
