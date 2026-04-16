#nullable enable

using System;
using System.Collections.Generic;
using System.Linq;
using Formatting = Newtonsoft.Json.Formatting;

namespace Semio;

/// <summary>In-place application of a <see cref="KitDiff"/> to a <see cref="Kit"/> (host-side, no persistence).</summary>
public static class KitInPlaceDiff
{
    public static void ApplyKitDiff(Kit kit, KitDiff diff)
    {
        if (diff.ShouldSerializeName()) kit.Name = diff.Name ?? "";
        if (diff.ShouldSerializeVersion()) kit.Version = diff.Version ?? "";
        if (diff.ShouldSerializeDescription()) kit.Description = diff.Description;
        if (diff.ShouldSerializeIcon()) kit.Icon = diff.Icon;
        if (diff.ShouldSerializeImage()) kit.Image = diff.Image;
        if (diff.ShouldSerializePreview()) kit.Preview = diff.Preview;
        if (diff.ShouldSerializeRemote()) kit.Remote = diff.Remote;
        if (diff.ShouldSerializeHomepage()) kit.Homepage = diff.Homepage;
        if (diff.ShouldSerializeLicense()) kit.License = diff.License;
        if (diff.ShouldSerializeCreatedAt()) kit.CreatedAt = diff.CreatedAt;
        if (diff.ShouldSerializeUpdatedAt()) kit.UpdatedAt = diff.UpdatedAt;

        if (diff.Types != null)
        {
            kit.Types ??= new List<Type>();
            ApplyTypesDiff(kit.Types, diff.Types);
        }

        if (diff.Designs != null)
        {
            kit.Designs ??= new List<Design>();
            ApplyDesignsDiff(kit.Designs, diff.Designs);
        }

        if (diff.Tags != null)
        {
            kit.Tags ??= new List<Tag>();
            ApplyTagsDiff(kit.Tags, diff.Tags);
        }

        if (diff.Folders != null)
        {
            kit.Folders ??= new List<Folder>();
            ApplyFoldersDiff(kit.Folders, diff.Folders);
        }

        if (diff.Ports != null)
        {
            kit.Ports ??= new List<Port>();
            ApplyPortsDiff(kit.Ports, diff.Ports);
        }

        if (diff.Concepts != null)
        {
            kit.Concepts ??= new List<Concept>();
            ApplyConceptsDiff(kit.Concepts, diff.Concepts);
        }

        if (diff.Files != null)
        {
            kit.Files ??= new List<File>();
            ApplyFilesDiff(kit.Files, diff.Files);
        }

        if (diff.Authors != null)
        {
            kit.Authors ??= new List<Author>();
            ApplyAuthorsDiff(kit.Authors, diff.Authors);
        }

        if (diff.Attributes != null)
        {
            kit.Attributes ??= new List<Attribute>();
            ApplyAttributesDiff(kit.Attributes, diff.Attributes);
        }
    }

    private static void ApplyTagsDiff(List<Tag> tags, TagsDiff diff)
    {
        if (diff.Removed != null)
            tags.RemoveAll(t => diff.Removed.Any(r => r.Id == t.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var tag = tags.FirstOrDefault(t => t.Id == update.Tag.Id);
                if (tag != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) tag.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) tag.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) tag.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            tags.AddRange(diff.Added);
    }

    private static void ApplyFoldersDiff(List<Folder> folders, FoldersDiff diff)
    {
        if (diff.Removed != null)
            folders.RemoveAll(f => diff.Removed.Any(r => r.Id == f.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var folder = folders.FirstOrDefault(f => f.Id == update.Folder.Id);
                if (folder != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) folder.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) folder.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeParent()) folder.Parent = update.Diff.Parent;
                }
            }
        }

        if (diff.Added != null)
            folders.AddRange(diff.Added);
    }

    private static void ApplyPortsDiff(List<Port> ports, PortsDiff diff)
    {
        if (diff.Removed != null)
            ports.RemoveAll(p => diff.Removed.Any(r => r.Id == p.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var port = ports.FirstOrDefault(p => p.Id == update.Port.Id);
                if (port != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) port.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) port.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) port.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeCompatiblePorts()) port.CompatiblePorts = update.Diff.CompatiblePorts;
                }
            }
        }

        if (diff.Added != null)
            ports.AddRange(diff.Added);
    }

    private static void ApplyConceptsDiff(List<Concept> concepts, ConceptsDiff diff)
    {
        if (diff.Removed != null)
            concepts.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var concept = concepts.FirstOrDefault(c => c.Id == update.Concept.Id);
                if (concept != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) concept.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) concept.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) concept.Icon = update.Diff.Icon;
                }
            }
        }

        if (diff.Added != null)
            concepts.AddRange(diff.Added);
    }

    private static void ApplyFilesDiff(List<File> files, FilesDiff diff)
    {
        if (diff.Removed != null)
            files.RemoveAll(f => diff.Removed.Any(r => r.Id == f.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var file = files.FirstOrDefault(f => f.Id == update.File.Id);
                if (file != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) file.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeRemote()) file.Remote = update.Diff.Remote;
                    if (update.Diff.ShouldSerializeFolder()) file.Folder = update.Diff.Folder;
                }
            }
        }

        if (diff.Added != null)
            files.AddRange(diff.Added);
    }

    private static void ApplyAuthorsDiff(List<Author> authors, AuthorsDiff diff)
    {
        if (diff.Removed != null)
            authors.RemoveAll(a => diff.Removed.Any(r => r.Id == a.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var author = authors.FirstOrDefault(a => a.Id == update.Author.Id);
                if (author != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) author.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeEmail()) author.Email = update.Diff.Email ?? "";
                }
            }
        }

        if (diff.Added != null)
            authors.AddRange(diff.Added);
    }

    private static void ApplyAttributesDiff(List<Attribute> attributes, AttributesDiff diff)
    {
        if (diff.Removed != null)
            attributes.RemoveAll(a => diff.Removed.Any(r => r.Id == a.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var attr = attributes.FirstOrDefault(a => a.Id == update.Attribute.Id);
                if (attr != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeValue()) attr.Value = update.Diff.Value;
                    if (update.Diff.ShouldSerializeDefinition()) attr.Definition = update.Diff.Definition;
                }
            }
        }

        if (diff.Added != null)
            attributes.AddRange(diff.Added);
    }

    private static void ApplyTypesDiff(List<Type> types, TypesDiff diff)
    {
        if (diff.Removed != null)
            types.RemoveAll(t => diff.Removed.Any(r => r.Id == t.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var type = types.FirstOrDefault(t => t.Id == update.Type.Id);
                if (type != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) type.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) type.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) type.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) type.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) type.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) type.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) type.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeStock()) type.Stock = update.Diff.Stock ?? type.Stock;
                    if (update.Diff.ShouldSerializeVirtual()) type.Virtual = update.Diff.Virtual ?? type.Virtual;
                    if (update.Diff.ShouldSerializeUnit()) type.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeLocation()) type.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) type.Authors = update.Diff.Authors?.Select(a => new AuthorId { Id = a.Id }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) type.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Id = c.Id }).ToList();
                    if (update.Diff.Connectors != null)
                    {
                        type.Connectors ??= new List<Connector>();
                        ApplyConnectorsDiff(type.Connectors, update.Diff.Connectors);
                    }
                    if (update.Diff.Representations != null)
                    {
                        type.Representations ??= new List<Representation>();
                        ApplyRepresentationsDiff(type.Representations, update.Diff.Representations);
                    }
                    if (update.Diff.Attributes != null)
                    {
                        type.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(type.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            types.AddRange(diff.Added);
    }

    private static void ApplyConnectorsDiff(List<Connector> connectors, ConnectorsDiff diff)
    {
        if (diff.Removed != null)
            connectors.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var connector = connectors.FirstOrDefault(c => c.Id == update.Connector.Id);
                if (connector != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) connector.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) connector.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializePort()) connector.Port = update.Diff.Port;
                    if (update.Diff.ShouldSerializeMandatory()) connector.Mandatory = update.Diff.Mandatory ?? connector.Mandatory;
                    if (update.Diff.ShouldSerializeT()) connector.T = update.Diff.T ?? connector.T;
                    if (update.Diff.ShouldSerializePoint())
                    {
                        var pd = update.Diff.Point;
                        var bp = connector.Point ?? new Point();
                        connector.Point = new Point { X = bp.X + (pd?.X ?? 0), Y = bp.Y + (pd?.Y ?? 0), Z = bp.Z + (pd?.Z ?? 0) };
                    }
                    if (update.Diff.ShouldSerializeDirection())
                    {
                        var dd = update.Diff.Direction;
                        var bd = connector.Direction ?? new Vector();
                        connector.Direction = new Vector { X = bd.X + (dd?.X ?? 0), Y = bd.Y + (dd?.Y ?? 0), Z = bd.Z + (dd?.Z ?? 0) };
                    }
                    if (update.Diff.Attributes != null)
                    {
                        connector.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(connector.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            connectors.AddRange(diff.Added);
    }

    private static void ApplyRepresentationsDiff(List<Representation> representations, RepresentationsDiff diff)
    {
        if (diff.Removed != null)
            representations.RemoveAll(m => diff.Removed.Any(r => r.Id == m.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var representation = representations.FirstOrDefault(m => m.Id == update.Representation.Id);
                if (representation != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) representation.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) representation.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeFile()) representation.File = update.Diff.File;
                    if (update.Diff.ShouldSerializeTags()) representation.Tags = update.Diff.Tags;
                    if (update.Diff.Attributes != null)
                    {
                        representation.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(representation.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            representations.AddRange(diff.Added);
    }

    private static void ApplyDesignsDiff(List<Design> designs, DesignsDiff diff)
    {
        if (diff.Removed != null)
            designs.RemoveAll(d => diff.Removed.Any(r => r.Id == d.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var design = designs.FirstOrDefault(d => d.Id == update.Design.Id);
                if (design != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) design.Name = update.Diff.Name ?? "";
                    if (update.Diff.ShouldSerializeDescription()) design.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeIcon()) design.Icon = update.Diff.Icon;
                    if (update.Diff.ShouldSerializeImage()) design.Image = update.Diff.Image;
                    if (update.Diff.ShouldSerializeParent()) design.Parent = update.Diff.Parent;
                    if (update.Diff.ShouldSerializeIsAbstract()) design.IsAbstract = update.Diff.IsAbstract;
                    if (update.Diff.ShouldSerializeFolder()) design.Folder = update.Diff.Folder;
                    if (update.Diff.ShouldSerializeCanScale()) design.CanScale = update.Diff.CanScale;
                    if (update.Diff.ShouldSerializeCanMirror()) design.CanMirror = update.Diff.CanMirror;
                    if (update.Diff.ShouldSerializeUnit()) design.Unit = update.Diff.Unit;
                    if (update.Diff.ShouldSerializeActiveLayer()) design.ActiveLayer = update.Diff.ActiveLayer;
                    if (update.Diff.ShouldSerializeLocation()) design.Location = update.Diff.Location;
                    if (update.Diff.ShouldSerializeAuthors()) design.Authors = update.Diff.Authors?.Select(a => new AuthorId { Id = a.Id }).ToList();
                    if (update.Diff.ShouldSerializeConcepts()) design.Concepts = update.Diff.Concepts?.Select(c => new ConceptId { Id = c.Id }).ToList();
                    if (update.Diff.Pieces != null)
                    {
                        design.Pieces ??= new List<Piece>();
                        ApplyPiecesDiff(design.Pieces, update.Diff.Pieces);
                    }
                    if (update.Diff.Connections != null)
                    {
                        design.Connections ??= new List<Connection>();
                        ApplyConnectionsDiff(design.Connections, update.Diff.Connections);
                    }
                    if (update.Diff.Attributes != null)
                    {
                        design.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(design.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            designs.AddRange(diff.Added);
    }

    private static void ApplyPiecesDiff(List<Piece> pieces, PiecesDiff diff)
    {
        if (diff.Removed != null)
            pieces.RemoveAll(p => diff.Removed.Any(r => r.Id == p.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var piece = pieces.FirstOrDefault(p => p.Id == update.Piece.Id);
                if (piece != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeName()) piece.Name = update.Diff.Name;
                    if (update.Diff.ShouldSerializeDescription()) piece.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeType()) piece.Type = update.Diff.Type;
                    if (update.Diff.ShouldSerializeDesign()) piece.Design = update.Diff.Design;
                    if (update.Diff.ShouldSerializePlane()) piece.Plane = update.Diff.Plane;
                    if (update.Diff.ShouldSerializeCenter()) piece.Center = update.Diff.Center;
                    if (update.Diff.ShouldSerializeScale()) piece.Scale = update.Diff.Scale;
                    if (update.Diff.ShouldSerializeMirrorPlane()) piece.MirrorPlane = update.Diff.MirrorPlane;
                    if (update.Diff.ShouldSerializeIsHidden()) piece.IsHidden = update.Diff.IsHidden;
                    if (update.Diff.ShouldSerializeIsLocked()) piece.IsLocked = update.Diff.IsLocked;
                    if (update.Diff.ShouldSerializeColor()) piece.Color = update.Diff.Color;
                    if (update.Diff.Attributes != null)
                    {
                        piece.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(piece.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            pieces.AddRange(diff.Added);
    }

    private static void ApplyConnectionsDiff(List<Connection> connections, ConnectionsDiff diff)
    {
        if (diff.Removed != null)
            connections.RemoveAll(c => diff.Removed.Any(r => r.Id == c.Id));

        if (diff.Updated != null)
        {
            foreach (var update in diff.Updated)
            {
                var connection = connections.FirstOrDefault(c => c.Id == update.Connection.Id);
                if (connection != null && update.Diff != null)
                {
                    if (update.Diff.ShouldSerializeConnected() && update.Diff.Connected != null)
                    {
                        var s = connection.Connected ?? new Side();
                        if (update.Diff.Connected.ShouldSerializePiece()) s.Piece = update.Diff.Connected.Piece;
                        if (update.Diff.Connected.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Connected.DesignPiece;
                        if (update.Diff.Connected.ShouldSerializeConnector()) s.Connector = update.Diff.Connected.Connector;
                        connection.Connected = s;
                    }
                    if (update.Diff.ShouldSerializeConnecting() && update.Diff.Connecting != null)
                    {
                        var s = connection.Connecting ?? new Side();
                        if (update.Diff.Connecting.ShouldSerializePiece()) s.Piece = update.Diff.Connecting.Piece;
                        if (update.Diff.Connecting.ShouldSerializeDesignPiece()) s.DesignPiece = update.Diff.Connecting.DesignPiece;
                        if (update.Diff.Connecting.ShouldSerializeConnector()) s.Connector = update.Diff.Connecting.Connector;
                        connection.Connecting = s;
                    }
                    if (update.Diff.ShouldSerializeDescription()) connection.Description = update.Diff.Description;
                    if (update.Diff.ShouldSerializeGap()) connection.Gap = connection.Gap + (update.Diff.Gap ?? 0f);
                    if (update.Diff.ShouldSerializeShift()) connection.Shift = connection.Shift + (update.Diff.Shift ?? 0f);
                    if (update.Diff.ShouldSerializeRise()) connection.Rise = connection.Rise + (update.Diff.Rise ?? 0f);
                    if (update.Diff.ShouldSerializeRotation()) connection.Rotation = connection.Rotation + (update.Diff.Rotation ?? 0f);
                    if (update.Diff.ShouldSerializeTurn()) connection.Turn = connection.Turn + (update.Diff.Turn ?? 0f);
                    if (update.Diff.ShouldSerializeTilt()) connection.Tilt = connection.Tilt + (update.Diff.Tilt ?? 0f);
                    if (update.Diff.ShouldSerializeU()) connection.U = (connection.U ?? 0f) + (update.Diff.U ?? 0f);
                    if (update.Diff.ShouldSerializeV()) connection.V = (connection.V ?? 0f) + (update.Diff.V ?? 0f);
                    if (update.Diff.Attributes != null)
                    {
                        connection.Attributes ??= new List<Attribute>();
                        ApplyAttributesDiff(connection.Attributes, update.Diff.Attributes);
                    }
                }
            }
        }

        if (diff.Added != null)
            connections.AddRange(diff.Added);
    }
}
