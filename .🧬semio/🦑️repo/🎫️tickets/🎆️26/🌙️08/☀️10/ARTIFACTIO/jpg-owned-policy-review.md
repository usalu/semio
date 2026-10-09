# JPEG Owned State and Physical Export Policy

Current source review found JpgSnapshot.re_encode_quality documented as the native encoder's Annex-K table scaling parameter. ChangeReEncodeQuality changes that field without changing the owned raster. This is a physical export policy still retained under semantic snapshot/diff/mutation authority. The runtime owner confirmed the classification and is moving quality selection to native I/O options while preserving native quality/profile support and updating contracts/fixtures.

JpgSnapshot.pixels is decoded RGBA rather than the entropy-coded JPEG scan. JpgSegment.data is documented as retention of APP/COM segments not otherwise modeled. This latter claim needs a separate current-source audit of known versus uninterpreted roles; historical native test success does not prove that separation.

No current JPEG architectural completion or new runtime pass is claimed from this inspection. Runtime lifecycle ownership repair remains the owner's preceding task.
