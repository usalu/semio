// #region 🔖Header
// [👤semio📚net🛅semio💻semioext](semiorepo://p/u/semio/b/l/net/fd/req/Semio/f/SemioExt.cs)
// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

using Newtonsoft.Json;
using QuikGraph;
using QuikGraph.Algorithms;
using QuikGraph.Algorithms.Search;
using QuikGraph.Algorithms.ConnectedComponents;

using System;
using System.Collections.Generic;
using System.Linq;

namespace Semio;

public static class SemioExt
{
    public static Tag FindTag(List<Tag> tags, string guid)
    {
        var tag = tags.FirstOrDefault(t => t.Guid == guid);
        if (tag == null) throw new Exception($"Tag {guid} not found in tags");
        return tag;
    }

    public static Concept FindConcept(List<Concept> concepts, string guid)
    {
        var concept = concepts.FirstOrDefault(c => c.Guid == guid);
        if (concept == null) throw new Exception($"Concept {guid} not found in concepts");
        return concept;
    }

    public static Model FindModel(List<Model> models, List<string> tagGuids)
    {
        var model = models.FirstOrDefault(m => tagGuids.All(id => m.Tags.Any(t => t.Guid == id)));
        if (model == null) throw new Exception($"Model with tags {string.Join(", ", tagGuids)} not found in models");
        return model;
    }

    public static Connector FindConnector(List<Connector> connectors, string connectorGuid)
    {
        var connector = connectors.FirstOrDefault(p => p.Guid == connectorGuid);
        if (connector == null) throw new Exception($"Connector {connectorGuid} not found in connectors");
        return connector;
    }

    public static Connector FindConnectorInType(Type type, string connectorGuid)
    {
        return FindConnector(type.Connectors ?? new List<Connector>(), connectorGuid);
    }

    public static Piece FindPiece(List<Piece> pieces, string pieceGuid)
    {
        var piece = pieces.FirstOrDefault(p => p.Guid == pieceGuid);
        if (piece == null) throw new Exception($"Piece {pieceGuid} not found in pieces");
        return piece;
    }

    public static Connection FindConnection(List<Connection> connections, string connectionGuid)
    {
        var connection = connections.FirstOrDefault(c => c.Guid == connectionGuid);
        if (connection == null) throw new Exception($"Connection {connectionGuid} not found in connections");
        return connection;
    }

    public static List<Connection> FindPieceConnections(List<Connection> connections, string pieceGuid)
    {
        return connections.Where(c => c.Connected.Piece.Guid == pieceGuid || c.Connecting.Piece.Guid == pieceGuid).ToList();
    }

    public static Connector? FindConnectorForPieceInConnection(Type type, Connection connection, string pieceGuid)
    {
        string? connectorGuid = connection.Connected.Piece.Guid == pieceGuid ? connection.Connected.Connector?.Guid : connection.Connecting.Connector?.Guid;
        if (string.IsNullOrEmpty(connectorGuid)) return null;
        return FindConnectorInType(type, connectorGuid);
    }

    public static Piece FindPieceInDesign(Design design, string pieceGuid)
    {
        return FindPiece(design.Pieces ?? new List<Piece>(), pieceGuid);
    }

    public static Connection FindConnectionInDesign(Design design, string connectionGuid)
    {
        return FindConnection(design.Connections ?? new List<Connection>(), connectionGuid);
    }

    public static List<Connection> FindConnectionsInDesign(Design design, List<string> connectionGuids)
    {
        return connectionGuids.Select(g => FindConnectionInDesign(design, g)).ToList();
    }

    public static List<Connection> FindPieceConnectionsInDesign(Design design, string pieceGuid)
    {
        return FindPieceConnections(design.Connections ?? new List<Connection>(), pieceGuid);
    }

    public static (Piece connecting, Piece connected) FindConnectionPiecesInDesign(Design design, Connection connection)
    {
        return (
            FindPieceInDesign(design, connection.Connecting.Piece.Guid),
            FindPieceInDesign(design, connection.Connected.Piece.Guid)
        );
    }

    public static List<Connection> FindStaleConnectionsInDesign(Design design)
    {
        return (design.Connections ?? new List<Connection>()).Where(c =>
        {
            try
            {
                FindPieceInDesign(design, c.Connected.Piece.Guid);
                FindPieceInDesign(design, c.Connecting.Piece.Guid);
                return false;
            }
            catch
            {
                return true;
            }
        }).ToList();
    }

    public static File FindFileInKit(Kit kit, string fileGuid)
    {
        var file = kit.Files?.FirstOrDefault(f => f.Guid == fileGuid);
        if (file == null) throw new Exception($"File {fileGuid} not found in kit {kit.Name}");
        return file;
    }

    public static Tag FindTagInKit(Kit kit, string tagGuid)
    {
        var tag = kit.Tags?.FirstOrDefault(t => t.Guid == tagGuid);
        if (tag == null) throw new Exception($"Tag {tagGuid} not found in kit {kit.Name}");
        return tag;
    }

    public static Concept FindConceptInKit(Kit kit, string conceptGuid)
    {
        var concept = kit.Concepts?.FirstOrDefault(c => c.Guid == conceptGuid);
        if (concept == null) throw new Exception($"Concept {conceptGuid} not found in kit {kit.Name}");
        return concept;
    }

    public static Type FindTypeInKit(Kit kit, string typeGuid)
    {
        var type = kit.Types?.FirstOrDefault(t => t.Guid == typeGuid);
        if (type == null) throw new Exception($"Type {typeGuid} not found in kit {kit.Name}");
        return type;
    }

    public static Design FindDesignInKit(Kit kit, string designGuid)
    {
        var design = kit.Designs?.FirstOrDefault(d => d.Guid == designGuid);
        if (design == null) throw new Exception($"Design {designGuid} not found in kit {kit.Name}");
        return design;
    }

    public static Port FindPortInKit(Kit kit, string portGuid)
    {
        var port = kit.Ports?.FirstOrDefault(p => p.Guid == portGuid);
        if (port == null) throw new Exception($"Port {portGuid} not found in kit {kit.Name}");
        return port;
    }

    public static Type FindPieceTypeInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesignInKit(kit, designGuid);
        var piece = FindPieceInDesign(design, pieceGuid);
        if (piece.Type == null) throw new Exception($"Piece {pieceGuid} has no type");
        return FindTypeInKit(kit, piece.Type.Guid);
    }

    public static Piece FindParentPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesignInKit(kit, designGuid);
        var connection = FindPieceConnectionsInDesign(design, pieceGuid).FirstOrDefault(c => c.Connecting.Piece.Guid == pieceGuid);
        if (connection == null) throw new Exception($"No parent piece found for piece {pieceGuid}");
        return FindPieceInDesign(design, connection.Connected.Piece.Guid);
    }

    public static Connection FindParentConnectionForPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesignInKit(kit, designGuid);
        var connection = FindPieceConnectionsInDesign(design, pieceGuid).FirstOrDefault(c => c.Connecting.Piece.Guid == pieceGuid);
        if (connection == null) throw new Exception($"No parent connection found for piece {pieceGuid}");
        return connection;
    }

    public static List<Piece> FindChildrenPiecesInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesignInKit(kit, designGuid);
        var connections = FindPieceConnectionsInDesign(design, pieceGuid).Where(c => c.Connected.Piece.Guid == pieceGuid);
        return connections.Select(c => FindPieceInDesign(design, c.Connecting.Piece.Guid)).ToList();
    }

    public static List<Connector> FindUsedConnectorsByPieceInDesign(Kit kit, string designGuid, string pieceGuid)
    {
        var design = FindDesignInKit(kit, designGuid);
        var piece = FindPieceInDesign(design, pieceGuid);
        var type = piece.Type != null ? FindTypeInKit(kit, piece.Type.Guid) : null;
        if (type == null) return new List<Connector>();

        var connections = FindPieceConnectionsInDesign(design, pieceGuid);
        var connectors = new List<Connector>();
        foreach (var connection in connections)
        {
            var connector = FindConnectorForPieceInConnection(type, connection, pieceGuid);
            if (connector != null) connectors.Add(connector);
        }
        return connectors;
    }

    public static Type[] FindReplacableTypesForPieceInDesign(Kit kit, string designGuid, string pieceGuid, string[]? variants = null)
    {
        var design = FindDesignInKit(kit, designGuid);
        var connections = FindPieceConnectionsInDesign(design, pieceGuid);
        var requiredConnectors = new List<Connector>();

        foreach (var connection in connections)
        {
            try
            {
                var otherPieceId = connection.Connected.Piece.Guid == pieceGuid ? connection.Connecting.Piece.Guid : connection.Connected.Piece.Guid;
                var otherPiece = FindPieceInDesign(design, otherPieceId);
                if (otherPiece.Type == null) continue;

                var otherType = FindTypeInKit(kit, otherPiece.Type.Guid);
                var otherPortId = connection.Connected.Piece.Guid == pieceGuid ? connection.Connecting.Connector?.Guid : connection.Connected.Connector?.Guid;
                var otherPort = FindConnectorInType(otherType, otherPortId ?? "");
                requiredConnectors.Add(otherPort);
            }
            catch
            {
                continue;
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Guid ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return requiredConnectors.Count == 0;

            return requiredConnectors.All(requiredConnector =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Guid == requiredConnector.Guid); // Simplified compatibility check
            });
        }).ToArray();
    }

    public static Type[] FindReplacableTypesForPiecesInDesign(Kit kit, string designGuid, string[] pieceGuids, string[]? variants = null)
    {
        var design = FindDesignInKit(kit, designGuid);
        var pieces = pieceGuids.Select(id => FindPieceInDesign(design, id)).ToList();
        var externalConnections = new List<(Connection connection, Connector requiredConnector)>();

        foreach (var piece in pieces)
        {
            var connections = FindPieceConnectionsInDesign(design, piece.Guid);
            foreach (var connection in connections)
            {
                var otherPieceId = connection.Connected.Piece.Guid == piece.Guid ? connection.Connecting.Piece.Guid : connection.Connected.Piece.Guid;
                if (!pieceGuids.Contains(otherPieceId))
                {
                    try
                    {
                        var otherPiece = FindPieceInDesign(design, otherPieceId);
                        if (otherPiece.Type == null) continue;

                        var otherType = FindTypeInKit(kit, otherPiece.Type.Guid);
                        var otherPortId = connection.Connected.Piece.Guid == piece.Guid ? connection.Connecting.Connector?.Guid : connection.Connected.Connector?.Guid;
                        var otherPort = FindConnectorInType(otherType, otherPortId ?? "");
                        externalConnections.Add((connection, otherPort));
                    }
                    catch
                    {
                        continue;
                    }
                }
            }
        }

        return (kit.Types ?? new List<Type>()).Where(replacementType =>
        {
            if (replacementType.IsAbstract ?? false) return false;
            if (variants != null && !variants.Contains(replacementType.Parent?.Guid ?? "")) return false;
            if (replacementType.Connectors == null || replacementType.Connectors.Count == 0) return externalConnections.Count == 0;

            return externalConnections.All(ec =>
            {
                return replacementType.Connectors.Any(replacementConnector => replacementConnector.Guid == ec.requiredConnector.Guid); // Simplified compatibility check
            });
        }).ToArray();
    }

    // Simplified FlattenDesign, porting the JS logic
    public static DesignDiff FlattenDesign(Kit kit, string designId)
    {
        var design = FindDesignInKit(kit, designId);
        if (design.Pieces == null || design.Pieces.Count == 0) return new DesignDiff();

        var typesDict = (kit.Types ?? new List<Type>()).ToDictionary(t => t.Guid);

        Type? GetType(string typeGuid) => typesDict.TryGetValue(typeGuid, out var t) ? t : null;

        Connector? GetConnector(Type? type, string? connectorGuid)
        {
            if (type == null) return null;

            if (string.IsNullOrEmpty(connectorGuid))
            {
                if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];
                if (!string.IsNullOrEmpty(type.Parent?.Guid))
                {
                    var parentType = GetType(type.Parent.Guid);
                    return GetConnector(parentType, connectorGuid);
                }
                return null;
            }

            if (type.Connectors != null && type.Connectors.Count > 0)
            {
                var connector = type.Connectors.FirstOrDefault(p => p.Guid == connectorGuid);
                if (connector != null) return connector;
            }

            if (!string.IsNullOrEmpty(type.Parent?.Guid))
            {
                var parentType = GetType(type.Parent.Guid);
                var connector = GetConnector(parentType, connectorGuid);
                if (connector != null) return connector;
            }

            if (type.Connectors != null && type.Connectors.Count > 0) return type.Connectors[0];

            return null;
        }

        // Deep copy design
        var flatDesignJson = JsonConvert.SerializeObject(design);
        var flatDesign = JsonConvert.DeserializeObject<Design>(flatDesignJson);
        if (flatDesign == null) return new DesignDiff();

        if (flatDesign.Pieces == null) flatDesign.Pieces = new List<Piece>();

        var piecePlanes = new Dictionary<string, Plane>();
        var pieceMap = new Dictionary<string, Piece>();
        foreach (var p in flatDesign.Pieces)
        {
            if (!string.IsNullOrEmpty(p.Guid)) pieceMap[p.Guid] = p;
        }

        var filteredConnections = (flatDesign.Connections ?? new List<Connection>()).Where(connection =>
        {
            var sourceId = connection.Connected.Piece.Guid;
            var targetId = connection.Connecting.Piece.Guid;
            var sourceExists = pieceMap.ContainsKey(sourceId);
            var targetExists = pieceMap.ContainsKey(targetId);
            return sourceExists && targetExists;
        }).ToList();

        // Very basic mock of QuikGraph usage for finding connected components and doing BFS
        var graph = new UndirectedGraph<string, Edge<string>>();
        foreach (var p in flatDesign.Pieces)
        {
            graph.AddVertex(p.Guid);
        }
        foreach (var c in filteredConnections)
        {
            graph.AddEdge(new Edge<string>(c.Connected.Piece.Guid, c.Connecting.Piece.Guid));
        }

        var algorithm = new ConnectedComponentsAlgorithm<string, Edge<string>>(graph);
        algorithm.Compute();

        var components = algorithm.Components;
        var componentDict = new Dictionary<int, List<string>>();
        foreach (var kvp in components)
        {
            if (!componentDict.ContainsKey(kvp.Value)) componentDict[kvp.Value] = new List<string>();
            componentDict[kvp.Value].Add(kvp.Key);
        }

        Piece SetAttributes(Piece piece, IEnumerable<(string key, string value)> newAttrs)
        {
            var updatedAttrs = piece.Attributes?.ToList() ?? new List<Attribute>();
            foreach (var newAttr in newAttrs)
            {
                var existingIndex = updatedAttrs.FindIndex(a => a.Key == newAttr.key);
                if (existingIndex >= 0)
                {
                    updatedAttrs[existingIndex].Value = newAttr.value;
                }
                else
                {
                    updatedAttrs.Add(new Attribute { Guid = Guid.NewGuid().ToString(), Key = newAttr.key, Value = newAttr.value });
                }
            }
            piece.Attributes = updatedAttrs;
            return piece;
        }

        foreach (var component in componentDict.Values)
        {
            var roots = component.Where(nodeId =>
            {
                var piece = pieceMap.TryGetValue(nodeId, out var p) ? p : null;
                return piece?.Plane != null;
            }).ToList();

            var rootNode = roots.Count > 0 ? roots[0] : (component.Count > 0 ? component[0] : null);
            if (string.IsNullOrEmpty(rootNode)) continue;

            var rootPiece = pieceMap[rootNode];
            if (string.IsNullOrEmpty(rootPiece.Guid)) continue;

            var updatedRootPiece = SetAttributes(rootPiece, new[]
            {
                ("semio.fixedPieceId", rootPiece.Guid),
                ("semio.depth", "0")
            });
            pieceMap[rootNode] = updatedRootPiece;

            Plane rootPlane;
            if (rootPiece.Plane != null)
            {
                rootPlane = rootPiece.Plane;
            }
            else
            {
                rootPlane = new Plane { XAxis = new Vector { X = 1, Y = 0, Z = 0 }, YAxis = new Vector { X = 0, Y = 1, Z = 0 }, Origin = new Point { X = 0, Y = 0, Z = 0 } };
            }

            piecePlanes[rootPiece.Guid] = rootPlane;
            var rootPieceIndex = flatDesign.Pieces.FindIndex(p => p.Guid == rootPiece.Guid);
            if (rootPieceIndex != -1)
            {
                flatDesign.Pieces[rootPieceIndex].Plane = rootPlane;
                if (flatDesign.Pieces[rootPieceIndex].Center == null)
                {
                    flatDesign.Pieces[rootPieceIndex].Center = new Coord { U = 0, V = 0 };
                }
            }

            // Using QuikGraph for BFS
            var bfs = new UndirectedBreadthFirstSearchAlgorithm<string, Edge<string>>(graph);
            var depths = new Dictionary<string, int>();
            depths[rootNode] = 0;

            bfs.TreeEdge += (sender, e) =>
            {
                var parentId = depths.ContainsKey(e.Source) ? e.Source : e.Target;
                var childId = parentId == e.Source ? e.Target : e.Source;
                depths[childId] = depths[parentId] + 1;

                var parentPiece = pieceMap.TryGetValue(parentId, out var pp) ? pp : null;
                var childPiece = pieceMap.TryGetValue(childId, out var cp) ? cp : null;
                if (parentPiece == null || childPiece == null || string.IsNullOrEmpty(parentPiece.Guid) || string.IsNullOrEmpty(childPiece.Guid)) return;

                if (piecePlanes.ContainsKey(childPiece.Guid)) return;

                if (!piecePlanes.TryGetValue(parentPiece.Guid, out var parentPlane)) return;

                var connection = filteredConnections.FirstOrDefault(c =>
                    (c.Connected.Piece.Guid == parentId && c.Connecting.Piece.Guid == childId) ||
                    (c.Connecting.Piece.Guid == parentId && c.Connected.Piece.Guid == childId));

                if (connection == null) return;

                var parentSide = connection.Connected.Piece.Guid == parentId ? connection.Connected : connection.Connecting;
                var childSide = connection.Connecting.Piece.Guid == childId ? connection.Connecting : connection.Connected;

                var parentType = parentPiece.Type != null ? GetType(parentPiece.Type.Guid) : null;
                var childType = childPiece.Type != null ? GetType(childPiece.Type.Guid) : null;

                var parentConnectorGuid = parentSide.Connector?.Guid;
                var childConnectorGuid = childSide.Connector?.Guid;
                var parentConnector = GetConnector(parentType, parentConnectorGuid);
                var childConnector = GetConnector(childType, childConnectorGuid);

                if (parentConnector == null || childConnector == null) return;
                if (parentConnector.Point == null || parentConnector.Direction == null || childConnector.Point == null || childConnector.Direction == null) return;

                var childPlane = Design.DefaultComputeChildPlane(
                    parentPlane,
                    parentConnector.Point,
                    parentConnector.Direction,
                    childConnector.Point,
                    childConnector.Direction,
                    connection.Gap,
                    connection.Shift,
                    connection.Rise,
                    connection.Rotation,
                    connection.Turn,
                    connection.Tilt
                );
                piecePlanes[childPiece.Guid] = childPlane;

                var radius = 2.697;
                var verticalVExtra = 1.0;
                var horizontalScale = 3.0633;
                var parentCenter = parentPiece.Center ?? new Coord { U = 0, V = 0 };
                var connectionU = connection.U;
                var connectionV = connection.V;

                float childU, childV;

                if (parentCenter.U == 0 && parentCenter.V == 0)
                {
                    var angle = 2 * Math.PI * (parentConnector.T);
                    childU = (float)(radius * Math.Sin(angle));
                    childV = (float)(radius * Math.Cos(angle));
                }
                else
                {
                    var isVerticalConnection = Math.Abs(parentConnector.Direction?.Z ?? 0) > 0.5;

                    if (isVerticalConnection)
                    {
                        childU = parentCenter.U + (float)connectionU;
                        childV = parentCenter.V + (float)connectionV + (float)verticalVExtra;
                    }
                    else
                    {
                        childU = parentCenter.U + (float)(connectionU * horizontalScale);
                        childV = parentCenter.V + (float)(connectionV * horizontalScale);
                    }
                }

                var childCenter = new Coord { U = (float)Math.Round(childU), V = (float)Math.Round(childV) };

                var fixedPieceId = parentPiece.Attributes?.FirstOrDefault(q => q.Key == "semio.fixedPieceId")?.Value ?? "";

                childPiece.Plane = childPlane;
                childPiece.Center = childCenter;

                var flatChildPiece = SetAttributes(childPiece, new[]
                {
                    ("semio.fixedPieceId", fixedPieceId),
                    ("semio.parentPieceId", parentPiece.Guid),
                    ("semio.depth", depths[childId].ToString())
                });
                pieceMap[childId] = flatChildPiece;
            };

            bfs.Compute(rootNode);
        }

        flatDesign.Pieces = flatDesign.Pieces.Select(p => pieceMap.TryGetValue(p.Guid ?? "", out var mapped) ? mapped : p).ToList();
        flatDesign.Connections = new List<Connection>();

        var updatedPieces = flatDesign.Pieces.Select(flatPiece =>
        {
            var originalPiece = design.Pieces?.FirstOrDefault(p => p.Guid == flatPiece.Guid);
            if (originalPiece == null) return null;

            var pieceDiff = new PieceDiff();
            bool hasChanges = false;

            if (flatPiece.Plane != null && JsonConvert.SerializeObject(flatPiece.Plane) != JsonConvert.SerializeObject(originalPiece.Plane))
            {
                pieceDiff.Plane = flatPiece.Plane;
                hasChanges = true;
            }

            if (flatPiece.Center != null && JsonConvert.SerializeObject(flatPiece.Center) != JsonConvert.SerializeObject(originalPiece.Center))
            {
                pieceDiff.Center = flatPiece.Center;
                hasChanges = true;
            }

            if (JsonConvert.SerializeObject(flatPiece.Attributes) != JsonConvert.SerializeObject(originalPiece.Attributes))
            {
                // Simple attribute diff
                pieceDiff.Attributes = flatPiece.Attributes.ToList();
                hasChanges = true;
            }

            if (!hasChanges) return null;

            return new PieceDiffUpdate
            {
                Piece = new PieceId { Guid = flatPiece.Guid },
                Diff = pieceDiff
            };
        }).Where(u => u != null).Cast<PieceDiffUpdate>().ToList();

        var removedConnections = (design.Connections ?? new List<Connection>()).Select(c => new ConnectionId { Connected = new Side { Piece = new PieceId { Guid = c.Connected.Piece.Guid } }, Connecting = new Side { Piece = new PieceId { Guid = c.Connecting.Piece.Guid } } }).ToList();

        var designDiff = new DesignDiff();
        if (updatedPieces.Count > 0)
        {
            designDiff.Pieces = new PiecesDiff { Updated = updatedPieces };
        }
        if (removedConnections.Count > 0)
        {
            designDiff.Connections = new ConnectionsDiff { Removed = removedConnections };
        }

        return designDiff;
    }


    public static DesignDiff ReplaceClusterWithDesign(Design originalDesign, List<string> clusterPieceIds, Design clusteredDesign, List<Connection> externalConnections)
    {
        var addedPieces = clusteredDesign.Pieces ?? new List<Piece>();
        var addedConnections = clusteredDesign.Connections ?? new List<Connection>();

        var addedClusteredConnections = externalConnections.Select(c =>
        {
            var newConnectionJson = JsonConvert.SerializeObject(c);
            var newConnection = JsonConvert.DeserializeObject<Connection>(newConnectionJson);
            if (newConnection != null)
            {
                newConnection.Guid = Guid.NewGuid().ToString();
            }
            return newConnection;
        }).Where(c => c != null).Cast<Connection>().ToList();

        addedConnections.AddRange(addedClusteredConnections);

        return new DesignDiff
        {
            Pieces = new PiecesDiff
            {
                Removed = clusterPieceIds.Select(id => new PieceId { Guid = id }).ToList(),
                Added = addedPieces
            },
            Connections = new ConnectionsDiff
            {
                Removed = (originalDesign.Connections ?? new List<Connection>())
                    .Where(c => clusterPieceIds.Contains(c.Connected.Piece.Guid) || clusterPieceIds.Contains(c.Connecting.Piece.Guid))
                    .Select(c => new ConnectionId { Connected = new Side { Piece = new PieceId { Guid = c.Connected.Piece.Guid } }, Connecting = new Side { Piece = new PieceId { Guid = c.Connecting.Piece.Guid } } })
                    .ToList(),
                Added = addedConnections
            }
        };
    }
}
