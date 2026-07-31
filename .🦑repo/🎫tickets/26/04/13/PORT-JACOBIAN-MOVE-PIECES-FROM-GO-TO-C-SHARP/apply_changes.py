#!/usr/bin/env python3
"""Apply Jacobian MovePiecesInDesign changes to Compose.cs on disk."""

import os

COMPOSE_CS = r"C:\git\compose\compose\net\Compose\Compose.cs"
_unused = os.path.join(
    os.path.dirname(__file__),
    "..",
    "..",
    "..",
    "..",
    "compose",
    "net",
    "Compose",
    "Compose.cs",
)
COMPOSE_CS = os.path.abspath(COMPOSE_CS)

OLD_METHOD = """    public static DesignDiff MovePiecesInDesign(Design design, Design pieces, MoveVector vector)
    {
        var designConnections = design.Connections;
        var selectedPieces = pieces.Pieces;
        var selectedGuids = new HashSet<string>(selectedPieces.Select(p => p.Guid));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in designConnections)
        {
            connectionByChild[conn.Connecting.Piece.Guid] = conn;
        }
        var fixedGuids = new HashSet<string>();
        foreach (var guid in selectedGuids)
        {
            if (!connectionByChild.ContainsKey(guid))
                fixedGuids.Add(guid);
        }
        var pieceMap = design.Pieces.ToDictionary(p => p.Guid);
        var pieceUpdates = new List<PieceDiffUpdate>();
        foreach (var guid in fixedGuids)
        {
            if (!pieceMap.TryGetValue(guid, out var piece) || piece.Plane == null) continue;
            var basePlane = piece.Plane;
            var t = MoveTranslationWorldFromPiecePlane(basePlane, vector);
            pieceUpdates.Add(new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = guid },
                Diff = new PieceDiff
                {
                    Plane = new Plane
                    {
                        Origin = new Point
                        {
                            X = basePlane.Origin.X + t.X,
                            Y = basePlane.Origin.Y + t.Y,
                            Z = basePlane.Origin.Z + t.Z,
                        },
                        XAxis = new Vector { X = basePlane.XAxis.X, Y = basePlane.XAxis.Y, Z = basePlane.XAxis.Z },
                        YAxis = new Vector { X = basePlane.YAxis.X, Y = basePlane.YAxis.Y, Z = basePlane.YAxis.Z },
                    },
                },
            });
        }
        var connectionUpdates = new List<ConnectionDiffUpdate>();
        foreach (var guid in selectedGuids)
        {
            if (fixedGuids.Contains(guid)) continue;
            var isDescendant = false;
            var current = guid;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentGuid = conn.Connected.Piece.Guid;
                if (selectedGuids.Contains(parentGuid))
                {
                    isDescendant = true;
                    break;
                }
                current = parentGuid;
            }
            if (isDescendant) continue;
            if (connectionByChild.TryGetValue(guid, out var parentConn))
            {
                connectionUpdates.Add(new ConnectionDiffUpdate
                {
                    Connection = new ConnectionId { Guid = parentConn.Guid },
                    Diff = new ConnectionDiff { Gap = vector.Gap, Shift = vector.Shift, Rise = vector.Rise },
                });
            }
        }
        var diff = new DesignDiff();
        if (pieceUpdates.Count > 0)
            diff.Pieces = new PiecesDiff { Updated = pieceUpdates };
        if (connectionUpdates.Count > 0)
            diff.Connections = new ConnectionsDiff { Updated = connectionUpdates };
        return diff;
    }"""

NEW_CODE = """    private static double[] MoveTranslationWorld(Plane plane, MoveVector mv)
    {
        var xAxis = new double[] { plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z };
        var yAxis = new double[] { plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z };
        NormalizeD(xAxis);
        NormalizeD(yAxis);
        var zAxis = CrossD(xAxis, yAxis);
        if (zAxis[0] * zAxis[0] + zAxis[1] * zAxis[1] + zAxis[2] * zAxis[2] < 1e-12)
            return new double[] { 0, 0, 0 };
        NormalizeD(zAxis);
        return new double[]
        {
            mv.Shift * xAxis[0] + mv.Gap * yAxis[0] + mv.Rise * zAxis[0],
            mv.Shift * xAxis[1] + mv.Gap * yAxis[1] + mv.Rise * zAxis[1],
            mv.Shift * xAxis[2] + mv.Gap * yAxis[2] + mv.Rise * zAxis[2],
        };
    }

    private static void NormalizeD(double[] v)
    {
        var len = Math.Sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
        if (len < 1e-12) return;
        v[0] /= len; v[1] /= len; v[2] /= len;
    }

    private static double[] CrossD(double[] a, double[] b) =>
        new double[] { a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0] };

    private static double DotD(double[] a, double[] b) =>
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

    private static Plane IdentityPlaneForStructuralMove() => new Plane
    {
        Origin = new Point { X = 0, Y = 0, Z = 0 },
        XAxis = new Vector { X = 1, Y = 0, Z = 0 },
        YAxis = new Vector { X = 0, Y = 1, Z = 0 },
    };

    private static Connector GetConnectorFromType(Dictionary<string, Type> typesDict, Type typ, string connectorGuid)
    {
        if (typ == null) return null;
        if (string.IsNullOrEmpty(connectorGuid))
        {
            if (typ.Connectors.Count > 0) return typ.Connectors[0];
            if (typ.Parent != null && typesDict.TryGetValue(typ.Parent.Guid, out var parentType))
                return GetConnectorFromType(typesDict, parentType, connectorGuid);
            return null;
        }
        foreach (var c in typ.Connectors)
            if (c.Guid == connectorGuid) return c;
        if (typ.Parent != null && typesDict.TryGetValue(typ.Parent.Guid, out var pt))
        {
            var found = GetConnectorFromType(typesDict, pt, connectorGuid);
            if (found != null) return found;
        }
        if (typ.Connectors.Count > 0) return typ.Connectors[0];
        return null;
    }

    private static void ConnectionPlacementTranslationBasis(Connector parentConnector, out double[] gapDir, out double[] shiftDir, out double[] raiseDir)
    {
        var parentDirection = new double[] { parentConnector.Direction?.X ?? 0, parentConnector.Direction?.Y ?? 1, parentConnector.Direction?.Z ?? 0 };
        NormalizeD(parentDirection);
        var yAxis = new System.Numerics.Vector3(0, 1, 0);
        var pDir = new System.Numerics.Vector3((float)parentDirection[0], (float)parentDirection[1], (float)parentDirection[2]);
        var parentConnectorQuat = CreateFromTwoVectors(yAxis, pDir);
        var parentRotationT = QuaternionToMatrix(parentConnectorQuat);
        var gapV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitY);
        gapDir = new double[] { gapV.X, gapV.Y, gapV.Z };
        NormalizeD(gapDir);
        var shiftV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitX);
        shiftDir = new double[] { shiftV.X, shiftV.Y, shiftV.Z };
        NormalizeD(shiftDir);
        var raiseV = ApplyMatrix4ToVec3(parentRotationT, System.Numerics.Vector3.UnitZ);
        raiseDir = new double[] { raiseV.X, raiseV.Y, raiseV.Z };
        NormalizeD(raiseDir);
    }

    private static double[] ChildConnectorOriginWorld(Plane parentPlane, Connector parentConnector, Connector childConnector, Connection connection)
    {
        var childPlane = DefaultComputeChildPlane(
            parentPlane,
            parentConnector.Point ?? new Point(),
            parentConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            childConnector.Point ?? new Point(),
            childConnector.Direction ?? new Vector { X = 0, Y = 1, Z = 0 },
            connection.Gap, connection.Shift, connection.Rise,
            connection.Rotation, connection.Turn, connection.Tilt);
        return new double[] { childPlane.Origin.X, childPlane.Origin.Y, childPlane.Origin.Z };
    }

    private static Connection ConnectionWithNumericDelta(Connection connection, string key, double delta)
    {
        var c = new Connection
        {
            Guid = connection.Guid,
            Connected = connection.Connected,
            Connecting = connection.Connecting,
            Description = connection.Description,
            Gap = connection.Gap,
            Shift = connection.Shift,
            Rise = connection.Rise,
            Rotation = connection.Rotation,
            Turn = connection.Turn,
            Tilt = connection.Tilt,
            U = connection.U,
            V = connection.V,
        };
        switch (key)
        {
            case "gap": c.Gap += delta; break;
            case "shift": c.Shift += delta; break;
            case "rise": c.Rise += delta; break;
            case "rotation": c.Rotation += delta; break;
            case "turn": c.Turn += delta; break;
            case "tilt": c.Tilt += delta; break;
        }
        return c;
    }

    private static double[] SolveConnectionOriginMinNorm(double[][] cols, double[] t)
    {
        if (cols.Length == 0) return null;
        var jjt = new double[9];
        for (int c = 0; c < 3; c++)
            for (int r = 0; r < 3; r++)
            {
                double s = 0;
                foreach (var col in cols) s += col[r] * col[c];
                jjt[r + c * 3] = s;
            }
        jjt[0] += 1e-14; jjt[4] += 1e-14; jjt[8] += 1e-14;
        var det = jjt[0] * (jjt[4] * jjt[8] - jjt[7] * jjt[5])
                - jjt[3] * (jjt[1] * jjt[8] - jjt[7] * jjt[2])
                + jjt[6] * (jjt[1] * jjt[5] - jjt[4] * jjt[2]);
        if (Math.Abs(det) < 1e-22) return null;
        var invDet = 1.0 / det;
        var inv = new double[9];
        inv[0] = (jjt[4] * jjt[8] - jjt[5] * jjt[7]) * invDet;
        inv[1] = (jjt[2] * jjt[7] - jjt[1] * jjt[8]) * invDet;
        inv[2] = (jjt[1] * jjt[5] - jjt[2] * jjt[4]) * invDet;
        inv[3] = (jjt[5] * jjt[6] - jjt[3] * jjt[8]) * invDet;
        inv[4] = (jjt[0] * jjt[8] - jjt[2] * jjt[6]) * invDet;
        inv[5] = (jjt[2] * jjt[3] - jjt[0] * jjt[5]) * invDet;
        inv[6] = (jjt[3] * jjt[7] - jjt[4] * jjt[6]) * invDet;
        inv[7] = (jjt[1] * jjt[6] - jjt[0] * jjt[7]) * invDet;
        inv[8] = (jjt[0] * jjt[4] - jjt[1] * jjt[3]) * invDet;
        if (double.IsInfinity(inv[0]) || double.IsNaN(inv[0])) return null;
        var u = new double[]
        {
            inv[0] * t[0] + inv[3] * t[1] + inv[6] * t[2],
            inv[1] * t[0] + inv[4] * t[1] + inv[7] * t[2],
            inv[2] * t[0] + inv[5] * t[1] + inv[8] * t[2],
        };
        var deltas = new double[cols.Length];
        for (int i = 0; i < cols.Length; i++)
            deltas[i] = cols[i][0] * u[0] + cols[i][1] * u[1] + cols[i][2] * u[2];
        return deltas;
    }

    private static ConnectionDiff ConnectionDiffTranslationFallback(Plane parentPlane, Connector parentConnector, double[] tw)
    {
        ConnectionPlacementTranslationBasis(parentConnector, out var gapDir, out var shiftDir, out var raiseDir);
        var dgap = DotD(tw, gapDir);
        var dshift = DotD(tw, shiftDir);
        var drise = DotD(tw, raiseDir);
        var res = new double[]
        {
            tw[0] - dgap * gapDir[0] - dshift * shiftDir[0] - drise * raiseDir[0],
            tw[1] - dgap * gapDir[1] - dshift * shiftDir[1] - drise * raiseDir[1],
            tw[2] - dgap * gapDir[2] - dshift * shiftDir[2] - drise * raiseDir[2],
        };
        var px = new double[] { parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z };
        var py = new double[] { parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z };
        var diff = new ConnectionDiff();
        const double eps = 1e-9;
        if (Math.Abs(dgap) > eps) diff.Gap = dgap;
        if (Math.Abs(dshift) > eps) diff.Shift = dshift;
        if (Math.Abs(drise) > eps) diff.Rise = drise;
        var pxSq = px[0] * px[0] + px[1] * px[1] + px[2] * px[2];
        var pySq = py[0] * py[0] + py[1] * py[1] + py[2] * py[2];
        if (pxSq > 1e-24 && pySq > 1e-24)
        {
            var pxN = new double[] { px[0] / Math.Sqrt(pxSq), px[1] / Math.Sqrt(pxSq), px[2] / Math.Sqrt(pxSq) };
            var pyN = new double[] { py[0] / Math.Sqrt(pySq), py[1] / Math.Sqrt(pySq), py[2] / Math.Sqrt(pySq) };
            var du = DotD(res, pxN);
            var dv = DotD(res, pyN);
            if (Math.Abs(du) > eps) diff.U = du;
            if (Math.Abs(dv) > eps) diff.V = dv;
        }
        return diff;
    }

    private static ConnectionDiff ConnectionDiffFromStructuralMoveVector(
        Plane parentPlane, Connector parentConnector, Connector childConnector,
        Connection connection, Plane childPlane, MoveVector vector)
    {
        var child = childPlane ?? IdentityPlaneForStructuralMove();
        var tw = MoveTranslationWorld(child, vector);
        var tSq = tw[0] * tw[0] + tw[1] * tw[1] + tw[2] * tw[2];
        if (tSq < 1e-24) return new ConnectionDiff();
        if (childConnector == null)
            return ConnectionDiffTranslationFallback(parentPlane, parentConnector, tw);

        var jacobianKeys = new[] { "gap", "shift", "rise", "rotation", "turn", "tilt" };
        var jacobianEps = new Dictionary<string, double>
        {
            { "gap", 1e-6 }, { "shift", 1e-6 }, { "rise", 1e-6 },
            { "rotation", 1e-4 }, { "turn", 1e-4 }, { "tilt", 1e-4 },
        };
        var o0 = ChildConnectorOriginWorld(parentPlane, parentConnector, childConnector, connection);
        var cols = new double[jacobianKeys.Length][];
        for (int i = 0; i < jacobianKeys.Length; i++)
        {
            var epsVal = jacobianEps[jacobianKeys[i]];
            var perturbed = ConnectionWithNumericDelta(connection, jacobianKeys[i], epsVal);
            var o1 = ChildConnectorOriginWorld(parentPlane, parentConnector, childConnector, perturbed);
            cols[i] = new double[] { (o1[0] - o0[0]) / epsVal, (o1[1] - o0[1]) / epsVal, (o1[2] - o0[2]) / epsVal };
        }
        var deltas = SolveConnectionOriginMinNorm(cols, tw);
        var diff = new ConnectionDiff();
        const double epsOut = 1e-9;
        if (deltas != null)
        {
            for (int i = 0; i < jacobianKeys.Length; i++)
            {
                if (Math.Abs(deltas[i]) > epsOut)
                {
                    var v = deltas[i];
                    switch (jacobianKeys[i])
                    {
                        case "gap": diff.Gap = v; break;
                        case "shift": diff.Shift = v; break;
                        case "rise": diff.Rise = v; break;
                        case "rotation": diff.Rotation = v; break;
                        case "turn": diff.Turn = v; break;
                        case "tilt": diff.Tilt = v; break;
                    }
                }
            }
            var pred = new double[] { 0, 0, 0 };
            for (int i = 0; i < cols.Length; i++)
            {
                pred[0] += cols[i][0] * deltas[i];
                pred[1] += cols[i][1] * deltas[i];
                pred[2] += cols[i][2] * deltas[i];
            }
            var res = new double[] { tw[0] - pred[0], tw[1] - pred[1], tw[2] - pred[2] };
            var px = new double[] { parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z };
            var py = new double[] { parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z };
            var pxSq = px[0] * px[0] + px[1] * px[1] + px[2] * px[2];
            var pySq = py[0] * py[0] + py[1] * py[1] + py[2] * py[2];
            if (pxSq > 1e-24 && pySq > 1e-24)
            {
                var pxN = new double[] { px[0] / Math.Sqrt(pxSq), px[1] / Math.Sqrt(pxSq), px[2] / Math.Sqrt(pxSq) };
                var pyN = new double[] { py[0] / Math.Sqrt(pySq), py[1] / Math.Sqrt(pySq), py[2] / Math.Sqrt(pySq) };
                var du = DotD(res, pxN);
                var dv = DotD(res, pyN);
                if (Math.Abs(du) > epsOut) diff.U = du;
                if (Math.Abs(dv) > epsOut) diff.V = dv;
            }
            return diff;
        }
        return ConnectionDiffTranslationFallback(parentPlane, parentConnector, tw);
    }

    public static DesignDiff MovePiecesInDesign(Kit kit, Design design, Design pieces, MoveVector vector)
    {
        var typesDict = new Dictionary<string, Type>();
        foreach (var t in kit.Types) typesDict[t.Guid] = t;

        var selectedGuids = new HashSet<string>(pieces.Pieces.Select(p => p.Guid));
        var connectionByChild = new Dictionary<string, Connection>();
        foreach (var conn in design.Connections)
            connectionByChild[conn.Connecting.Piece.Guid] = conn;

        var fixedGuids = new HashSet<string>();
        foreach (var guid in selectedGuids)
            if (!connectionByChild.ContainsKey(guid))
                fixedGuids.Add(guid);

        var pieceMap = design.Pieces.ToDictionary(p => p.Guid);
        var pieceUpdates = new List<PieceDiffUpdate>();
        foreach (var guid in fixedGuids)
        {
            if (!pieceMap.TryGetValue(guid, out var piece) || piece.Plane == null) continue;
            var basePlane = piece.Plane;
            var t = MoveTranslationWorldFromPiecePlane(basePlane, vector);
            pieceUpdates.Add(new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = guid },
                Diff = new PieceDiff
                {
                    Plane = new Plane
                    {
                        Origin = new Point
                        {
                            X = basePlane.Origin.X + t.X,
                            Y = basePlane.Origin.Y + t.Y,
                            Z = basePlane.Origin.Z + t.Z,
                        },
                        XAxis = new Vector { X = basePlane.XAxis.X, Y = basePlane.XAxis.Y, Z = basePlane.XAxis.Z },
                        YAxis = new Vector { X = basePlane.YAxis.X, Y = basePlane.YAxis.Y, Z = basePlane.YAxis.Z },
                    },
                },
            });
        }
        var connectionUpdates = new List<ConnectionDiffUpdate>();
        foreach (var guid in selectedGuids)
        {
            if (fixedGuids.Contains(guid)) continue;
            var isDescendant = false;
            var current = guid;
            while (connectionByChild.TryGetValue(current, out var conn))
            {
                var parentGuid = conn.Connected.Piece.Guid;
                if (selectedGuids.Contains(parentGuid)) { isDescendant = true; break; }
                current = parentGuid;
            }
            if (isDescendant) continue;
            if (!connectionByChild.TryGetValue(guid, out var parentConn)) continue;
            var parentPiece = pieceMap.GetValueOrDefault(parentConn.Connected.Piece.Guid);
            var childPiece = pieceMap.GetValueOrDefault(guid);
            if (parentPiece == null || childPiece == null) continue;
            if (parentPiece.Type == null || childPiece.Type == null) continue;
            typesDict.TryGetValue(parentPiece.Type.Guid, out var parentType);
            typesDict.TryGetValue(childPiece.Type.Guid, out var childType);
            var parentConnector = GetConnectorFromType(typesDict, parentType,
                parentConn.Connected.Connector?.Guid ?? "");
            var childConnector = GetConnectorFromType(typesDict, childType,
                parentConn.Connecting.Connector?.Guid ?? "");
            if (parentConnector == null) continue;
            var parentPlane = parentPiece.Plane ?? IdentityPlaneForStructuralMove();
            var connDiff = ConnectionDiffFromStructuralMoveVector(
                parentPlane, parentConnector, childConnector,
                parentConn, childPiece.Plane, vector);
            var hasFields = connDiff.Gap.HasValue || connDiff.Shift.HasValue || connDiff.Rise.HasValue ||
                connDiff.Rotation.HasValue || connDiff.Turn.HasValue || connDiff.Tilt.HasValue ||
                connDiff.U.HasValue || connDiff.V.HasValue;
            if (!hasFields) continue;
            connectionUpdates.Add(new ConnectionDiffUpdate
            {
                Connection = new ConnectionId { Guid = parentConn.Guid },
                Diff = connDiff,
            });
        }
        var diff = new DesignDiff();
        if (pieceUpdates.Count > 0)
            diff.Pieces = new PiecesDiff { Updated = pieceUpdates };
        if (connectionUpdates.Count > 0)
            diff.Connections = new ConnectionsDiff { Updated = connectionUpdates };
        return diff;
    }"""

print(f"Reading {COMPOSE_CS}")
with open(COMPOSE_CS, "r", encoding="utf-8") as f:
    content = f.read()

idx = content.find(OLD_METHOD)
if idx == -1:
    print("ERROR: Could not find old MovePiecesInDesign method in file!")
    exit(1)

print(f"Found old method at character offset {idx}")
new_content = content[:idx] + NEW_CODE + content[idx + len(OLD_METHOD) :]

with open(COMPOSE_CS, "w", encoding="utf-8") as f:
    f.write(new_content)

print("SUCCESS: Replaced old MovePiecesInDesign with Jacobian-based version")
print(f"Old method: {len(OLD_METHOD)} chars")
print(f"New code: {len(NEW_CODE)} chars")

# Verify
with open(COMPOSE_CS, "r", encoding="utf-8") as f:
    verify = f.read()
if "MovePiecesInDesign(Kit kit" in verify and "NormalizeD" in verify:
    print("VERIFIED: New code is on disk")
else:
    print("ERROR: Verification failed!")
    exit(1)
