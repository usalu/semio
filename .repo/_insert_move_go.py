# Temporary script; inserts MovePiecesInDesign before DragPiecesInDesign.
path = r"c:\git\semio\semio\go\main.go"
with open(path, "r", encoding="utf-8") as f:
    lines = f.readlines()
needle = "func DragPiecesInDesign(design Design, pieces Design, offset Coord) DesignDiff {"
idx = next(i for i, l in enumerate(lines) if needle in l)
start = idx - 2
if not lines[start].strip().startswith("//"):
    raise SystemExit(f"unexpected line {start+1}: {lines[start]!r}")
insert = r"""// MoveVector carries gap/shift/rise deltas in the piece plane frame (gap along yAxis, shift along xAxis, rise along normal).
type MoveVector struct {
	Gap   float64 `json:"gap"`
	Shift float64 `json:"shift"`
	Rise  float64 `json:"rise"`
}

func movePlaneOriginDelta(plane *Plane, mv MoveVector) PointDiff {
	if plane == nil {
		return PointDiff{}
	}
	xAxis := []float64{plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z}
	yAxis := []float64{plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z}
	normalize(xAxis)
	normalize(yAxis)
	zAxis := cross(xAxis, yAxis)
	normalize(zAxis)
	tx := mv.Shift*xAxis[0] + mv.Gap*yAxis[0] + mv.Rise*zAxis[0]
	ty := mv.Shift*xAxis[1] + mv.Gap*yAxis[1] + mv.Rise*zAxis[1]
	tz := mv.Shift*xAxis[2] + mv.Gap*yAxis[2] + mv.Rise*zAxis[2]
	nx := plane.Origin.X + tx
	ny := plane.Origin.Y + ty
	nz := plane.Origin.Z + tz
	return PointDiff{X: &nx, Y: &ny, Z: &nz}
}

// MovePiecesInDesign computes a DesignDiff that translates root piece planes and adjusts gap/shift/rise on parent connections for selected child movers.
// A piece's parent connection is the connection where it is the Connecting (child) piece.
func MovePiecesInDesign(design Design, pieces Design, vector MoveVector) DesignDiff {
	selectedGuids := make(map[string]bool)
	for _, p := range pieces.Pieces {
		selectedGuids[p.Guid] = true
	}
	parentMap := make(map[string]struct{ connectionGuid, parentGuid string })
	for _, c := range design.Connections {
		parentMap[c.Connecting.Piece.Guid] = struct{ connectionGuid, parentGuid string }{c.Guid, c.Connected.Piece.Guid}
	}
	fixedGuids := make(map[string]bool)
	for guid := range selectedGuids {
		if _, hasParent := parentMap[guid]; !hasParent {
			fixedGuids[guid] = true
		}
	}
	var pieceUpdates []struct {
		Piece PieceId   `json:"piece"`
		Diff  PieceDiff `json:"diff"`
	}
	pieceMap := make(map[string]*Piece)
	for i := range design.Pieces {
		pieceMap[design.Pieces[i].Guid] = &design.Pieces[i]
	}
	for guid := range fixedGuids {
		p, ok := pieceMap[guid]
		if !ok || p.Plane == nil {
			continue
		}
		orig := movePlaneOriginDelta(p.Plane, vector)
		pieceUpdates = append(pieceUpdates, struct {
			Piece PieceId   `json:"piece"`
			Diff  PieceDiff `json:"diff"`
		}{
			Piece: PieceId{Guid: guid},
			Diff:  PieceDiff{Plane: &PlaneDiff{Origin: &orig}},
		})
	}
	var connectionUpdates []struct {
		Connection ConnectionId   `json:"connection"`
		Diff       ConnectionDiff `json:"diff"`
	}
	for guid := range selectedGuids {
		if fixedGuids[guid] {
			continue
		}
		isDescendant := false
		current := guid
		for {
			p, ok := parentMap[current]
			if !ok {
				break
			}
			if selectedGuids[p.parentGuid] {
				isDescendant = true
				break
			}
			current = p.parentGuid
		}
		if isDescendant {
			continue
		}
		parent, ok := parentMap[guid]
		if !ok {
			continue
		}
		g, s, r := vector.Gap, vector.Shift, vector.Rise
		connectionUpdates = append(connectionUpdates, struct {
			Connection ConnectionId   `json:"connection"`
			Diff       ConnectionDiff `json:"diff"`
		}{
			Connection: ConnectionId{Guid: parent.connectionGuid},
			Diff:       ConnectionDiff{Gap: &g, Shift: &s, Rise: &r},
		})
	}
	diff := DesignDiff{}
	if len(pieceUpdates) > 0 {
		diff.Pieces = &PiecesDiff{Updated: pieceUpdates}
	}
	if len(connectionUpdates) > 0 {
		diff.Connections = &ConnectionsDiff{Updated: connectionUpdates}
	}
	return diff
}

"""
newlines = lines[:start] + [insert] + lines[start:]
with open(path, "w", encoding="utf-8", newline="\n") as f:
    f.writelines(newlines)
print("ok", start + 1)
