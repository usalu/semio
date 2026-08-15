# PPTX Office Deflate Primary Research

## Question

Can OPC or ZIP metadata determine the exact compressor decisions needed to reproduce the supplied PowerPoint archive without persisting physical state?

## Primary-source findings

- Microsoft's OPC overview defines the package as a logical collection mapped to a ZIP-based physical organization, but does not prescribe a byte-canonical DEFLATE encoder. It explicitly separates logical package concepts from physical package mapping. Source: [Microsoft OPC overview](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/opc/open-packaging-conventions-overview).
- `System.IO.Packaging` exposes a semantic compression option for package parts, not a persisted DEFLATE match-search or block-layout model. Source: [Microsoft System.IO.Packaging](https://learn.microsoft.com/en-us/dotnet/api/system.io.packaging?view=netframework-4.8).
- Microsoft's open `ZipPackage` implementation maps semantic `CompressionOption.Normal` to `CompressionLevel.Optimal`, `Maximum` to `SmallestSize` on modern .NET (otherwise `Optimal`), and Fast/SuperFast to `Fastest`. It also notes that imported ZIP entries do not reveal their original compression level, reinforcing that compression policy is serializer behavior rather than package state. Source: [Microsoft ZipPackage source](https://source.dot.net/System.IO.Packaging/System/IO/Packaging/ZipPackage.cs.html).
- Relationship parts inherit their source part's semantic compression option in Microsoft's packaging implementation. Source: [Microsoft InternalRelationshipCollection source](https://source.dot.net/System.IO.Packaging/System/IO/Packaging/InternalRelationshipCollection.cs.html).
- The miniz compressor's public flags cover probe count, greedy parsing, match filtering, forced raw/static blocks, RLE, and an explicitly nondeterministic initialization mode. These are implementation controls, not OPC logical state. Source: [miniz source reference](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/doxyhtml/miniz_8h_source.html).

## Consequence for this ticket

Exact fixture reconstruction must select one deterministic compressor policy from logical PPTX/OPC properties and implement its DEFLATE decisions in the serializer. Compression method, flags, probe counts, imported block boundaries, and member order must not be persisted in snapshot, artifact, diff, mutation, or facets. The current fixture-backed block trace is therefore the acceptance oracle for the deterministic writer, not data to retain in schema state.
