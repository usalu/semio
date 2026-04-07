// #region 🔖ExportDesignModel/Helpers

// exportPlaneToGltfMatrix converts a Plane to a column-major 4x4 matrix for gltf.Node.Matrix.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportplanetogltfmatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportPlaneToGltfMatrix)
func exportPlaneToGltfMatrix(plane Plane) [16]float64 {
	ox, oy, oz := plane.Origin.X, plane.Origin.Y, plane.Origin.Z
	xx, xy, xz := plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z
	yx, yy, yz := plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z
	zx := xy*yz - xz*yy
	zy := xz*yx - xx*yz
	zz := xx*yy - xy*yx
	zLen := math.Sqrt(zx*zx + zy*zy + zz*zz)
	if zLen > 0 {
		zx /= zLen
		zy /= zLen
		zz /= zLen
	}
	xLen := math.Sqrt(xx*xx + xy*xy + xz*xz)
	if xLen > 0 {
		xx /= xLen
		xy /= xLen
		xz /= xLen
	}
	yx = zy*xz - zz*xy
	yy = zz*xx - zx*xz
	yz = zx*xy - zy*xx
	yLen := math.Sqrt(yx*yx + yy*yy + yz*yz)
	if yLen > 0 {
		yx /= yLen
		yy /= yLen
		yz /= yLen
	}
	return [16]float64{xx, xy, xz, 0, yx, yy, yz, 0, zx, zy, zz, 0, ox, oy, oz, 1}
}

// exportDenseToGltfMatrix converts a gonum mat.Dense (row-major) to column-major glTF matrix.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdensetogltfmatrix](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportDenseToGltfMatrix)
func exportDenseToGltfMatrix(m *mat.Dense) [16]float64 {
	return [16]float64{
		m.At(0, 0), m.At(1, 0), m.At(2, 0), m.At(3, 0),
		m.At(0, 1), m.At(1, 1), m.At(2, 1), m.At(3, 1),
		m.At(0, 2), m.At(1, 2), m.At(2, 2), m.At(3, 2),
		m.At(0, 3), m.At(1, 3), m.At(2, 3), m.At(3, 3),
	}
}

// exportDecodeBlobToBytes strips a data URI prefix and base64 decodes the blob content.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdecodeblobtobytes](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportDecodeBlobToBytes)
func exportDecodeBlobToBytes(blob string) ([]byte, error) {
	if idx := strings.Index(blob, ","); idx >= 0 {
		blob = blob[idx+1:]
	}
	decoded, err := base64.StdEncoding.DecodeString(blob)
	if err != nil {
		decoded, err = base64.RawStdEncoding.DecodeString(blob)
		if err != nil {
			return nil, fmt.Errorf("base64 decode failed: %w", err)
		}
	}
	return decoded, nil
}

// exportCreateBox generates a unit box placeholder mesh in the gltf document and returns the mesh index.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportcreatebox](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportCreateBox)
func exportCreateBox(doc *gltf.Document) int {
	positions := [][3]float32{
		{-0.5, -0.5, -0.5}, {0.5, -0.5, -0.5}, {0.5, 0.5, -0.5}, {-0.5, 0.5, -0.5},
		{-0.5, -0.5, 0.5}, {0.5, -0.5, 0.5}, {0.5, 0.5, 0.5}, {-0.5, 0.5, 0.5},
	}
	indices := []uint16{
		0, 1, 2, 0, 2, 3,
		4, 6, 5, 4, 7, 6,
		0, 4, 5, 0, 5, 1,
		3, 2, 6, 3, 6, 7,
		0, 3, 7, 0, 7, 4,
		1, 5, 6, 1, 6, 2,
	}
	posIdx := modeler.WritePosition(doc, positions)
	idxIdx := modeler.WriteIndices(doc, indices)
	meshIdx := len(doc.Meshes)
	doc.Meshes = append(doc.Meshes, &gltf.Mesh{
		Name: "placeholder",
		Primitives: []*gltf.Primitive{{
			Indices:    gltf.Index(idxIdx),
			Attributes: gltf.PrimitiveAttributes{gltf.POSITION: posIdx},
		}},
	})
	return meshIdx
}

// exportCopyMeshFromGLB decodes a GLB and copies the first mesh's geometry into the target document.
// Returns the mesh index.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportcopymeshfromglb](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportCopyMeshFromGLB)
func exportCopyMeshFromGLB(doc *gltf.Document, glbData []byte, meshName string) (int, error) {
	srcDoc := new(gltf.Document)
	if err := gltf.NewDecoder(bytes.NewReader(glbData)).Decode(srcDoc); err != nil {
		return 0, fmt.Errorf("failed to decode GLB: %w", err)
	}
	if len(srcDoc.Meshes) == 0 {
		return 0, fmt.Errorf("no meshes in source GLB")
	}
	srcMesh := srcDoc.Meshes[0]
	newPrimitives := make([]*gltf.Primitive, 0, len(srcMesh.Primitives))
	for _, srcPrim := range srcMesh.Primitives {
		attrs := gltf.PrimitiveAttributes{}
		if posAccIdx, ok := srcPrim.Attributes[gltf.POSITION]; ok {
			if posAccIdx < len(srcDoc.Accessors) {
				positions, err := modeler.ReadPosition(srcDoc, srcDoc.Accessors[posAccIdx], nil)
				if err == nil && len(positions) > 0 {
					attrs[gltf.POSITION] = modeler.WritePosition(doc, positions)
				}
			}
		}
		if normAccIdx, ok := srcPrim.Attributes[gltf.NORMAL]; ok {
			if normAccIdx < len(srcDoc.Accessors) {
				normals, err := modeler.ReadNormal(srcDoc, srcDoc.Accessors[normAccIdx], nil)
				if err == nil && len(normals) > 0 {
					attrs[gltf.NORMAL] = modeler.WriteNormal(doc, normals)
				}
			}
		}
		if tcAccIdx, ok := srcPrim.Attributes[gltf.TEXCOORD_0]; ok {
			if tcAccIdx < len(srcDoc.Accessors) {
				texcoords, err := modeler.ReadTextureCoord(srcDoc, srcDoc.Accessors[tcAccIdx], nil)
				if err == nil && len(texcoords) > 0 {
					attrs[gltf.TEXCOORD_0] = modeler.WriteTextureCoord(doc, texcoords)
				}
			}
		}
		newPrim := &gltf.Primitive{
			Attributes: attrs,
		}
		if srcPrim.Indices != nil && *srcPrim.Indices < len(srcDoc.Accessors) {
			indices, err := modeler.ReadIndices(srcDoc, srcDoc.Accessors[*srcPrim.Indices], nil)
			if err == nil && len(indices) > 0 {
				newPrim.Indices = gltf.Index(modeler.WriteIndices(doc, indices))
			}
		}
		newPrimitives = append(newPrimitives, newPrim)
	}
	meshIdx := len(doc.Meshes)
	doc.Meshes = append(doc.Meshes, &gltf.Mesh{
		Name:       meshName,
		Primitives: newPrimitives,
	})
	return meshIdx, nil
}

// exportFindModelForKind finds the best matching model for a type given tag filters.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportfindmodelforkind](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/exportFindModelForKind)
func exportFindModelForKind(typ *Type, tags []string, tagsDict map[string]*Tag) *Model {
	if len(typ.Models) == 0 {
		return nil
	}
	if len(tags) == 0 {
		return &typ.Models[0]
	}
	tagNameSet := make(map[string]bool)
	for _, t := range tags {
		tagNameSet[t] = true
	}
	bestModel := &typ.Models[0]
	bestCount := 0
	for i := range typ.Models {
		model := &typ.Models[i]
		matchCount := 0
		for _, tid := range model.Tags {
			if tag, ok := tagsDict[tid.Guid]; ok {
				if tagNameSet[tag.Name] {
					matchCount++
				}
			}
		}
		if matchCount == len(tags) {
			return model
		}
		if matchCount > bestCount {
			bestCount = matchCount
			bestModel = model
		}
	}
	return bestModel
}

// #endregion 🔖ExportDesignModel/Helpers

// ExportDesignModel exports the 3D model of a design to GLB or glTF format.
// [👤semio📚go💻semio🔖exportdesignmodel🛠️exportdesignmodel](repo://p/u/semio/b/l/go/f/semio.go/s/Export%20Design%20Model/d/i/ExportDesignModel)
func ExportDesignModel(kit *Kit, designGuid string, format string, tags []string, options map[string]interface{}) ([]byte, error) {
	if _, ok := ExportModelFormats[format]; !ok {
		return nil, fmt.Errorf("unsupported format: %s", format)
	}

	design := FindDesignInKit(kit, designGuid)
	if design == nil {
		return nil, fmt.Errorf("design not found: %s", designGuid)
	}
	if len(design.Pieces) == 0 {
		return nil, fmt.Errorf("design has no pieces")
	}

	typesDict := make(map[string]*Type)
	for i := range kit.Types {
		typesDict[kit.Types[i].Guid] = &kit.Types[i]
	}
	filesDict := make(map[string]*File)
	for i := range kit.Files {
		filesDict[kit.Files[i].Guid] = &kit.Files[i]
	}
	tagsDict := make(map[string]*Tag)
	for i := range kit.Tags {
		tagsDict[kit.Tags[i].Guid] = &kit.Tags[i]
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Guid] = &design.Pieces[i]
	}

	// #region 🔖ExportDesignModel/BFS
	piecePlanes := make(map[string]*Plane)
	parentOf := make(map[string]string)
	childrenOf := make(map[string][]string)
	var rootPieceGuids []string

	adjacency := make(map[string][]struct {
		neighborGuid string
		connection   *Connection
	})
	for i := range design.Connections {
		conn := &design.Connections[i]
		srcGuid := conn.Connected.Piece.Guid
		tgtGuid := conn.Connecting.Piece.Guid
		if pieceMap[srcGuid] == nil || pieceMap[tgtGuid] == nil {
			continue
		}
		adjacency[srcGuid] = append(adjacency[srcGuid], struct {
			neighborGuid string
			connection   *Connection
		}{tgtGuid, conn})
		adjacency[tgtGuid] = append(adjacency[tgtGuid], struct {
			neighborGuid string
			connection   *Connection
		}{srcGuid, conn})
	}

	visited := make(map[string]bool)
	var bfsExport func(rootGuid string)
	bfsExport = func(rootGuid string) {
		queue := []string{rootGuid}
		visited[rootGuid] = true
		rootPieceGuids = append(rootPieceGuids, rootGuid)
		rootPiece := pieceMap[rootGuid]
		if rootPiece.Plane != nil {
			piecePlanes[rootGuid] = rootPiece.Plane
		} else {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			piecePlanes[rootGuid] = &p
		}
		for len(queue) > 0 {
			currentGuid := queue[0]
			queue = queue[1:]
			currentPlane := piecePlanes[currentGuid]
			currentPiece := pieceMap[currentGuid]

			for _, neighbor := range adjacency[currentGuid] {
				if visited[neighbor.neighborGuid] {
					continue
				}
				visited[neighbor.neighborGuid] = true
				neighborPiece := pieceMap[neighbor.neighborGuid]
				conn := neighbor.connection

				var parentSide, childSide *Side
				if conn.Connected.Piece.Guid == currentGuid {
					parentSide = &conn.Connected
					childSide = &conn.Connecting
				} else {
					parentSide = &conn.Connecting
					childSide = &conn.Connected
				}

				var parentType, childType *Type
				if currentPiece.Type != nil {
					parentType = typesDict[currentPiece.Type.Guid]
				}
				if neighborPiece.Type != nil {
					childType = typesDict[neighborPiece.Type.Guid]
				}

				var parentConnectorGuid, childConnectorGuid *string
				if parentSide.Connector != nil {
					parentConnectorGuid = &parentSide.Connector.Guid
				}
				if childSide.Connector != nil {
					childConnectorGuid = &childSide.Connector.Guid
				}

				parentConnector := getConnector(typesDict, parentType, parentConnectorGuid)
				childConnector := getConnector(typesDict, childType, childConnectorGuid)
				if parentConnector == nil || childConnector == nil {
					continue
				}

				childPlane := computeChildPlane(*currentPlane, *parentConnector, *childConnector, *conn)
				piecePlanes[neighbor.neighborGuid] = &childPlane
				parentOf[neighbor.neighborGuid] = currentGuid
				childrenOf[currentGuid] = append(childrenOf[currentGuid], neighbor.neighborGuid)

				queue = append(queue, neighbor.neighborGuid)
			}
		}
	}
	for _, piece := range design.Pieces {
		if !visited[piece.Guid] {
			bfsExport(piece.Guid)
		}
	}
	// #endregion 🔖ExportDesignModel/BFS

	// #region 🔖ExportDesignModel/BuildGLTF
	doc := gltf.NewDocument()
	doc.Asset.Generator = "semio"

	usedTypes := make(map[string]bool)
	for _, piece := range design.Pieces {
		if piece.Type != nil {
			usedTypes[piece.Type.Guid] = true
		}
	}
	typeOrder := make([]string, 0, len(usedTypes))
	for typeGuid := range usedTypes {
		typeOrder = append(typeOrder, typeGuid)
	}
	sort.Strings(typeOrder)

	typeMeshIndex := make(map[string]int)
	for _, typeGuid := range typeOrder {
		typ := typesDict[typeGuid]
		if typ == nil {
			typeMeshIndex[typeGuid] = exportCreateBox(doc)
			continue
		}
		model := exportFindModelForKind(typ, tags, tagsDict)
		if model == nil {
			typeMeshIndex[typeGuid] = exportCreateBox(doc)
			continue
		}
		file := filesDict[model.File.Guid]
		if file == nil || file.Blob == nil || *file.Blob == "" {
			typeMeshIndex[typeGuid] = exportCreateBox(doc)
			continue
		}
		glbData, err := exportDecodeBlobToBytes(*file.Blob)
		if err != nil || len(glbData) < 4 {
			typeMeshIndex[typeGuid] = exportCreateBox(doc)
			continue
		}
		meshIdx, err := exportCopyMeshFromGLB(doc, glbData, typ.Name)
		if err != nil {
			typeMeshIndex[typeGuid] = exportCreateBox(doc)
			continue
		}
		typeMeshIndex[typeGuid] = meshIdx
	}

	pieceNodeIndex := make(map[string]int)
	for i, piece := range design.Pieces {
		pieceNodeIndex[piece.Guid] = i
	}

	for _, piece := range design.Pieces {
		plane := piecePlanes[piece.Guid]
		if plane == nil {
			p := Plane{
				Origin: Point{X: 0, Y: 0, Z: 0},
				XAxis:  Vector{X: 1, Y: 0, Z: 0},
				YAxis:  Vector{X: 0, Y: 1, Z: 0},
			}
			plane = &p
		}

		var matrix [16]float64
		if parentGuid, hasParent := parentOf[piece.Guid]; hasParent {
			parentPlane := piecePlanes[parentGuid]
			if parentPlane != nil {
				parentMat := planeToMatrix(*parentPlane)
				childMat := planeToMatrix(*plane)
				var inv mat.Dense
				if err := inv.Inverse(parentMat); err == nil {
					var relative mat.Dense
					relative.Mul(&inv, childMat)
					matrix = exportDenseToGltfMatrix(&relative)
				} else {
					matrix = exportPlaneToGltfMatrix(*plane)
				}
			} else {
				matrix = exportPlaneToGltfMatrix(*plane)
			}
		} else {
			matrix = exportPlaneToGltfMatrix(*plane)
		}

		name := piece.Guid
		if piece.Name != nil && *piece.Name != "" {
			name = *piece.Name
		}

		node := &gltf.Node{
			Name:   name,
			Matrix: matrix,
		}
		if piece.Type != nil {
			if idx, ok := typeMeshIndex[piece.Type.Guid]; ok {
				node.Mesh = gltf.Index(idx)
			}
		}
		for _, childGuid := range childrenOf[piece.Guid] {
			if idx, ok := pieceNodeIndex[childGuid]; ok {
				node.Children = append(node.Children, idx)
			}
		}
		doc.Nodes = append(doc.Nodes, node)
	}

	var sceneRootNodes []int
	for _, rootGuid := range rootPieceGuids {
		if idx, ok := pieceNodeIndex[rootGuid]; ok {
			sceneRootNodes = append(sceneRootNodes, idx)
		}
	}
	doc.Scenes[0].Nodes = sceneRootNodes

	var buf bytes.Buffer
	enc := gltf.NewEncoder(&buf)
	if format == ".gltf" {
		enc.AsBinary = false
	}
	if err := enc.Encode(doc); err != nil {
		return nil, fmt.Errorf("failed to encode glTF: %w", err)
	}
	return buf.Bytes(), nil
	// #endregion 🔖ExportDesignModel/BuildGLTF
}

// #endregion 🔖ExportDesignModel
