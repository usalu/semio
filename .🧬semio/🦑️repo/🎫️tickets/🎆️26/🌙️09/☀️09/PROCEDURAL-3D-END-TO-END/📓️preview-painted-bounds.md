# Camera fit from the visible preview buffers

Edit, generate and view now fit from the same typed position and edge buffers they publish after the show-mode filter. This removes a full parse of the serialized mesh document in each preview render. The viewer retains the measured bounds with its geometry table, so a warm camera or hover update does not scan the buffers. A finite coordinate triple contributes as a unit. Incomplete and nonfinite triples cannot manufacture partial bounds.

Seven language-neutral cases cover several meshes, edge-only and point-only payloads, incomplete buffers and nonfinite components. The isolated production Rust implementation and the independent Three.js Box3 and Float32BufferAttribute check each passed all seven. The linked window composition remains pending. The three mode call sites use the typed bounds and keep the existing fit revision and padding.
