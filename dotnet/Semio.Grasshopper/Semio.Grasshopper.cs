using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using GH_IO.Serialization;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino;
using Rhino.Geometry;
using Semio.Grasshopper.Properties;
using static System.Windows.Forms.VisualStyles.VisualStyleElement.ListView;

namespace Semio.Grasshopper;

// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();

#region Copilot

//public class Representation : IDeepCloneable<global::Representation>
//{
//    public string Url { get; set; }
//    public string? Lod { get; set; }
//    public List<string>? Tags { get; set; }

//    public global::Representation DeepClone()
//    {
//        var representation = new global::Representation
//        {
//            Url = Url
//        };
//        if (Lod != null) representation.Lod = Lod;
//        if (Tags != null) representation.Tags = new List<string>(Tags);
//        return representation;
//    }

//    public override string ToString()
//    {
//        return $"Representation(Url: {Url})";
//    }
//}

//public class Specifier : IDeepCloneable<global::Specifier>
//{
//    public string Context { get; set; }
//    public string Group { get; set; }

//    public global::Specifier DeepClone()
//    {
//        return new global::Specifier
//        {
//            Context = Context,
//            Group = Group
//        };
//    }

//    public override string ToString()
//    {
//        return $"Specifier(Context: {Context})";
//    }
//}

//public class ScreenPoint : IDeepCloneable<ScreenPoint>
//{
//    public int X { get; set; }
//    public int Y { get; set; }

//    public ScreenPoint DeepClone()
//    {
//        return new ScreenPoint
//        {
//            X = X,
//            Y = Y
//        };
//    }

//    public override string ToString()
//    {
//        return $"Point(X: {X}, Y: {Y})";
//    }
//}

//public class Point : IDeepCloneable<global::Point>
//{
//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//    public global::Point DeepClone()
//    {
//        return new global::Point
//        {
//            X = X,
//            Y = Y,
//            Z = Z
//        };
//    }

//    public override string ToString()
//    {
//        return $"Point(X: {X}, Y: {Y}, Z: {Z})";
//    }
//}

//public class Vector : IDeepCloneable<global::Vector>
//{
//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//    public global::Vector DeepClone()
//    {
//        return new global::Vector
//        {
//            X = X,
//            Y = Y,
//            Z = Z
//        };
//    }

//    public override string ToString()
//    {
//        return $"Vector(X: {X}, Y: {Y}, Z: {Z})";
//    }
//}

//public class Plane : IDeepCloneable<global::Plane>
//{
//    public global::Point Origin { get; set; }
//    public global::Vector XAxis { get; set; }
//    public global::Vector YAxis { get; set; }

//    public global::Plane DeepClone()
//    {
//        return new global::Plane
//        {
//            Origin = Origin.DeepClone(),
//            XAxis = XAxis.DeepClone(),
//            YAxis = YAxis.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Plane(Origin: {Origin}, XAxis: {XAxis}, YAxis: {YAxis})";
//    }
//}

//public class Port : IDeepCloneable<global::Port>
//{
//    public global::Plane Plane { get; set; }
//    public List<global::Specifier> Specifiers { get; set; }

//    public global::Port DeepClone()
//    {
//        return new global::Port
//        {
//            Plane = Plane.DeepClone(),
//            Specifiers = new List<global::Specifier>(Specifiers.Select(s => s.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Port({GetHashCode()})";
//    }
//}

//public class PortId : IDeepCloneable<global::PortId>
//{
//    public List<global::Specifier> Specifiers { get; set; }

//    public global::PortId DeepClone()
//    {
//        return new global::PortId
//        {
//            Specifiers = new List<global::Specifier>(Specifiers.Select(s => s.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"PortId({GetHashCode()})";
//    }
//}


//public class Quality : IDeepCloneable<global::Quality>
//{
//    public string Name { get; set; }
//    public string Value { get; set; }
//    public string? Unit { get; set; }

//    public global::Quality DeepClone()
//    {
//        var quality = new global::Quality
//        {
//            Name = Name,
//            Value = Value
//        };
//        if (Unit != null) quality.Unit = Unit;
//        return quality;
//    }

//    public override string ToString()
//    {
//        return $"Quality(Name: {Name})";
//    }
//}

//public class Type : IDeepCloneable<global::Type>
//{
//    public string Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Variant { get; set; }
//    public string Unit { get; set; }
//    public List<global::Representation> Representations { get; set; }
//    public List<global::Port> Ports { get; set; }
//    public List<global::Quality>? Qualities { get; set; }

//    public global::Type DeepClone()
//    {
//        var type = new global::Type
//        {
//            Name = Name,
//            Unit = Unit,
//            Representations = new List<global::Representation>(Representations.Select(r => r.DeepClone())),
//            Ports = new List<global::Port>(Ports.Select(p => p.DeepClone()))
//        };
//        if (Description != null) type.Description = Description;
//        if (Icon != null) type.Icon = Icon;
//        if (Variant != null) type.Variant = Variant;
//        if (Qualities != null) type.Qualities = new List<global::Quality>(Qualities.Select(q => q.DeepClone()));
//        return type;
//    }

//    public override string ToString()
//    {
//        return $"Type(Name: {Name}, Variant: {Variant})";
//    }
//}

//public class TypeId : IDeepCloneable<global::TypeId>
//{
//    public string Name { get; set; }
//    public string? Variant { get; set; }

//    public global::TypeId DeepClone()
//    {
//        var typeId = new global::TypeId
//        {
//            Name = Name
//        };
//        if (Variant != null) typeId.Variant = Variant;
//        return typeId;
//    }

//    public override string ToString()
//    {
//        return $"TypeId(Name: {Name}, Variant: {Variant})";
//    }
//}

//public class RootPiece : IDeepCloneable<global::RootPiece>
//{
//    public global::Plane Plane { get; set; }

//    public global::RootPiece DeepClone()
//    {
//        return new global::RootPiece
//        {
//            Plane = Plane.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"RootPiece({GetHashCode()})";
//    }
//}

//public class DiagramPiece : IDeepCloneable<global::DiagramPiece>
//{
//    public global::ScenePoint Point { get; set; }

//    public global::DiagramPiece DeepClone()
//    {
//        return new global::DiagramPiece
//        {
//            Point = Point.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"DiagramPiece({GetHashCode()})";
//    }
//}

//public class Piece : IDeepCloneable<global::Piece>
//{
//    public string Id { get; set; }
//    public global::TypeId Type { get; set; }
//    public global::RootPiece? Root { get; set; }
//    public global::DiagramPiece Diagram { get; set; }
//    public global::Piece DeepClone()
//    {
//        var piece = new global::Piece
//        {
//            Id = Id,
//            Type = Type.DeepClone(),
//            Diagram = Diagram.DeepClone()
//        };
//        if (Root != null) piece.Root = Root.DeepClone();
//        return piece;
//    }

//    public override string ToString()
//    {
//        return $"Piece(Id: {Id})";
//    }
//}

//public class PieceId : IDeepCloneable<global::PieceId>
//{
//    public string Id { get; set; }

//    public global::PieceId DeepClone()
//    {
//        return new global::PieceId
//        {
//            Id = Id
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceId(Id: {Id})";
//    }
//}

//public class TypePieceSide : IDeepCloneable<global::TypePieceSide>
//{
//    public global::PortId Port { get; set; }

//    public global::TypePieceSide DeepClone()
//    {
//        return new global::TypePieceSide
//        {
//            Port = Port.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"TypePieceSide({GetHashCode()})";
//    }
//}

//public class PieceSide : IDeepCloneable<global::PieceSide>
//{
//    public string Id { get; set; }
//    public global::TypePieceSide Type { get; set; }

//    public global::PieceSide DeepClone()
//    {
//        return new global::PieceSide
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceSide(Id: {Id})";
//    }
//}

//public class Side : IDeepCloneable<global::Side>
//{
//    public global::PieceSide Piece { get; set; }

//    public global::Side DeepClone()
//    {
//        return new global::Side
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Side({GetHashCode()})";
//    }
//}


//public class Attraction : IDeepCloneable<global::Attraction>
//{
//    public global::Side Attracting { get; set; }
//    public global::Side Attracted { get; set; }

//    public global::Attraction DeepClone()
//    {
//        return new global::Attraction
//        {
//            Attracting = Attracting.DeepClone(),
//            Attracted = Attracted.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Attraction(Attracting(Piece: {Attracting.Piece.Id}), Attracted(Piece: {Attracted.Piece.Id}))";
//    }
//}

//public class Formation : IDeepCloneable<global::Formation>
//{
//    public string Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Variant { get; set; }
//    public string Unit { get; set; }
//    public List<global::Piece> Pieces { get; set; }
//    public List<global::Attraction> Attractions { get; set; }
//    public List<global::Quality>? Qualities { get; set; }

//    public global::Formation DeepClone()
//    {
//        var formation = new global::Formation
//        {
//            Name = Name,
//            Unit = Unit,
//            Pieces = new List<global::Piece>(Pieces.Select(p => p.DeepClone())),
//            Attractions = new List<global::Attraction>(Attractions.Select(a => a.DeepClone()))
//        };
//        if (Description != null) formation.Description = Description;
//        if (Icon != null) formation.Icon = Icon;
//        if (Variant != null) formation.Variant = Variant;
//        if (Qualities != null) formation.Qualities = new List<global::Quality>(Qualities.Select(q => q.DeepClone()));
//        return formation;
//    }

//    public override string ToString()
//    {
//        return $"Formation(Name: {Name}, Variant: {Variant})";
//    }
//}

//public class FormationId : IDeepCloneable<global::FormationId>
//{
//    public string Name { get; set; }
//    public string? Variant { get; set; }

//    public global::FormationId DeepClone()
//    {
//        var formationId = new global::FormationId
//        {
//            Name = Name
//        };
//        if (Variant != null) formationId.Variant = Variant;
//        return formationId;
//    }

//    public override string ToString()
//    {
//        return $"FormationId(Name: {Name}, Variant: {Variant})";
//    }
//}

//public class TypePieceObject : IDeepCloneable<global::TypePieceObject>
//{
//    public List<global::Representation> Representations { get; set; }

//    public global::TypePieceObject DeepClone()
//    {
//        return new global::TypePieceObject
//        {
//            Representations = new List<global::Representation>(Representations.Select(f => f.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"TypePieceObject({GetHashCode()})";
//    }
//}

//public class PieceObject : IDeepCloneable<global::PieceObject>
//{
//    public string Id { get; set; }
//    public global::TypePieceObject Type { get; set; }

//    public global::PieceObject DeepClone()
//    {
//        return new global::PieceObject
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceObject(Id: {Id})";
//    }
//}


//public class ParentObject : IDeepCloneable<global::ParentObject>
//{
//    public global::PieceId Piece { get; set; }

//    public global::ParentObject DeepClone()
//    {
//        return new global::ParentObject
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"ParentObject({GetHashCode()})";
//    }
//}

//public class Object : IDeepCloneable<global::Object>
//{
//    public global::PieceObject Piece { get; set; }
//    public global::Plane Plane { get; set; }

//    public global::ParentObject? Parent { get; set; }

//    public global::Object DeepClone()
//    {
//        var obj = new global::Object
//        {
//            Piece = Piece.DeepClone(),
//            Plane = Plane.DeepClone()
//        };
//        if (Parent != null) obj.Parent = Parent.DeepClone();
//        return obj;
//    }

//    public override string ToString()
//    {
//        return $"Object({GetHashCode()})";
//    }
//}

//public class Scene : IDeepCloneable<global::Scene>
//{
//    public List<global::Object> Objects { get; set; }

//    public global::Scene DeepClone()
//    {
//        return new global::Scene
//        {
//            Objects = new List<global::Object>(Objects.Select(o => o.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Scene({GetHashCode()})";
//    }
//}

//public class Kit : IDeepCloneable<global::Kit>
//{
//    public string Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Url { get; set; }
//    public List<global::Type> Types { get; set; }
//    public List<global::Formation> Formations { get; set; }

//    public global::Kit DeepClone()
//    {
//        var kit = new global::Kit
//        {
//            Name = Name,
//            Types = new List<global::Type>(Types.Select(t => t.DeepClone())),
//            Formations = new List<global::Formation>(Formations.Select(f => f.DeepClone()))
//        };
//        if (Description != null) kit.Description = Description;

//        if (Icon != null) kit.Icon = Icon;

//        if (Url != null) kit.Url = Url;
//        return kit;
//    }

//    public override string ToString()
//    {
//        return $"Kit(Name: {Name}, {GetHashCode()})";
//    }
//}

//public class KitMetadata : IDeepCloneable<global::KitMetadata>
//{
//    public string? Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Url { get; set; }

//    public global::KitMetadata DeepClone()
//    {
//        var kitMetadata = new global::KitMetadata();
//        if (Name != null) kitMetadata.Name = Name;
//        if (Description != null) kitMetadata.Description = Description;
//        if (Icon != null) kitMetadata.Icon = Icon;
//        if (Url != null) kitMetadata.Url = Url;
//        return kitMetadata;
//    }

//    public override string ToString()
//    {
//        return $"KitMetadata(Name: {Name})";
//    }
//}
#endregion

#region Utility

public static class Utility
{
    public static string ServerErrorMessage =>
        "The server seems to have a problem with this.\nTry to restart the server and try it again and if it happens again please report this on GitHub:\n https://github.com/usalu/semio";

    public static bool IsPathFullyQualified(string path)
    {
        return Path.GetFullPath(path) == path;
    }

    public static bool IsValidUnit(string unit)
    {
        return new string[] { "nm", "mm", "cm", "dm", "m", "km", "µin", "in", "ft", "yd"}.Contains(unit);
    }
    public static string UnitSystemToAbbreviation(UnitSystem unitSystem)
    {
        var unit = unitSystem switch
        {
            UnitSystem.Nanometers => "nm",
            UnitSystem.Millimeters => "mm",
            UnitSystem.Centimeters => "cm",
            UnitSystem.Decimeters => "dm",
            UnitSystem.Meters => "m",
            UnitSystem.Kilometers => "km",
            UnitSystem.Microinches => "µin",
            UnitSystem.Inches => "in",
            UnitSystem.Feet => "ft",
            UnitSystem.Yards => "yd",
            _ => "unsupported unit system"
        };
        if (IsValidUnit(unit) == false)
            throw new ArgumentException("Invalid unit system", nameof(unitSystem));
        return unit;
}
}

#endregion

#region Converters

public static class RhinoConverter
{
    public static Rhino.Geometry.Plane convert(this Plane plane)
    {
        return new Rhino.Geometry.Plane(
            new Point3d(plane.Origin.X, plane.Origin.Y, plane.Origin.Z),
            new Vector3d(plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z),
            new Vector3d(plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z)
        );
    }

    public static Plane convert(this Rhino.Geometry.Plane plane)
    {
        return new Plane
        {
            Origin = new Point
            {
                X = (float)plane.OriginX,
                Y = (float)plane.OriginY,
                Z = (float)plane.OriginZ
            },
            XAxis = new Vector
            {
                X = (float)plane.XAxis.X,
                Y = (float)plane.XAxis.Y,
                Z = (float)plane.XAxis.Z
            },
            YAxis = new Vector
            {
                X = (float)plane.YAxis.X,
                Y = (float)plane.YAxis.Y,
                Z = (float)plane.YAxis.Z
            }
        };
    }
}

#endregion

#region Goos

public class RepresentationGoo : GH_Goo<Representation>
{
    // TODO: Implement casts with string
    public RepresentationGoo()
    {
        Value = new Representation();
    }

    public RepresentationGoo(Representation representation)
    {
        Value = representation;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Representation";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new RepresentationGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Representation", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Representation").Deserialize<Representation>();
        return base.Read(reader);
    }
}

public class SpecifierGoo : GH_Goo<Specifier>
{
    public SpecifierGoo()
    {
        Value = new Specifier();
    }

    public SpecifierGoo(Specifier specifier)
    {
        Value = specifier;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Specifier";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new SpecifierGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Specifier", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Specifier").Deserialize<Specifier>();
        return base.Read(reader);
    }
}

public class PortGoo : GH_Goo<Port>
{
    public PortGoo()
    {
        Value = new Port();
    }

    public PortGoo(Port port)
    {
        Value = port;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Port";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new PortGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            object ptr = new GH_Plane(new Rhino.Geometry.Plane(
                new Point3d(Value.Plane.Origin.X, Value.Plane.Origin.Y, Value.Plane.Origin.Z),
                new Vector3d(Value.Plane.XAxis.X, Value.Plane.XAxis.Y, Value.Plane.XAxis.Z),
                new Vector3d(Value.Plane.YAxis.X, Value.Plane.YAxis.Y, Value.Plane.YAxis.Z)
            ));
            target = (Q)ptr;
            return true;
        }

        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Serialize());
            target = (Q)ptr;
            return true;
        }

        return false;
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;

        var plane = new Rhino.Geometry.Plane();
        if (GH_Convert.ToPlane(source, ref plane, GH_Conversion.Both))
        {
            Value.Plane = new Plane
            {
                Origin = new Point
                {
                    X = (float)plane.OriginX,
                    Y = (float)plane.OriginY,
                    Z = (float)plane.OriginZ
                },
                XAxis = new Vector
                {
                    X = (float)plane.XAxis.X,
                    Y = (float)plane.XAxis.Y,
                    Z = (float)plane.XAxis.Z
                },
                YAxis = new Vector
                {
                    X = (float)plane.YAxis.X,
                    Y = (float)plane.YAxis.Y,
                    Z = (float)plane.YAxis.Z
                }
            };
            return true;
        }

        string str = null;
        if (GH_Convert.ToString(source, out str, GH_Conversion.Both))
        {
            Value.Plane = str.Deserialize<Plane>();
            return true;
        }

        return false;
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Port", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Port").Deserialize<Port>();
        return base.Read(reader);
    }
}

public class QualityGoo : GH_Goo<Quality>
{
    public QualityGoo()
    {
        Value = new Quality();
    }

    public QualityGoo(Quality quality)
    {
        Value = quality;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Quality";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new QualityGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Quality", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Quality").Deserialize<Quality>();
        return base.Read(reader);
    }
}

public class TypeGoo : GH_Goo<Type>
{
    public TypeGoo()
    {
        Value = new Type();
    }

    public TypeGoo(Type type)
    {
        Value = type;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Type";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new TypeGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Type", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Type").Deserialize<Type>();
        return base.Read(reader);
    }
}

public class ScreenPointGoo : GH_Goo<ScreenPoint>
{
    public ScreenPointGoo()
    {
        Value = new ScreenPoint();
    }

    public ScreenPointGoo(ScreenPoint screenPoint)
    {
        Value = screenPoint;
    }

    public override bool IsValid { get; }
    public override string TypeName => "ScreenPoint";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new ScreenPointGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("ScreenPoint", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("ScreenPoint").Deserialize<ScreenPoint>();
        return base.Read(reader);
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            object ptr = new GH_Point(new Point3d(Value.X, Value.Y, 0));
            target = (Q)ptr;
            return true;
        }

        return false;
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;

        var point = new Point3d();
        if (GH_Convert.ToPoint3d(source, ref point, GH_Conversion.Both))
        {
            Value = new ScreenPoint
            {
                X = (int)point.X,
                Y = (int)point.Y
            };
            return true;
        }

        return false;
    }
}

public class PieceGoo : GH_Goo<Piece>
{
    public PieceGoo()
    {
        Value = new Piece();
    }

    public PieceGoo(Piece piece)
    {
        Value = piece;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Piece";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new PieceGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Piece", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Piece").Deserialize<Piece>();
        return base.Read(reader);
    }
}

public class SideGoo : GH_Goo<Side>
{
    public SideGoo()
    {
        Value = new Side();
    }

    public SideGoo(Side side)
    {
        Value = side;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Side";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new SideGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Side", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Side").Deserialize<Side>();
        return base.Read(reader);
    }
}

public class AttractionGoo : GH_Goo<Attraction>
{
    public AttractionGoo()
    {
        Value = new Attraction();
    }

    public AttractionGoo(Attraction attraction)
    {
        Value = attraction;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Attraction";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new AttractionGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Attraction", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Attraction").Deserialize<Attraction>();
        return base.Read(reader);
    }
}

// TODO: Implement cast with type
public class FormationGoo : GH_Goo<Formation>
{
    public FormationGoo()
    {
        Value = new Formation();
    }

    public FormationGoo(Formation formation)
    {
        Value = formation;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Formation";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new FormationGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Formation", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Formation").Deserialize<Formation>();
        return base.Read(reader);
    }
}

public class SceneGoo : GH_Goo<Scene>
{
    public SceneGoo()
    {
        Value = new Scene();
    }

    public SceneGoo(Scene scene)
    {
        Value = scene;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Scene";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new SceneGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Scene", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Scene").Deserialize<Scene>();
        return base.Read(reader);
    }
}

public class KitGoo : GH_Goo<Kit>
{
    public KitGoo()
    {
        Value = new Kit();
    }

    public KitGoo(Kit kit)
    {
        Value = kit;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Kit";
    public override string TypeDescription { get; }


    public override IGH_Goo Duplicate()
    {
        return new KitGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Kit", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Kit").Deserialize<Kit>();
        return base.Read(reader);
    }
}

#endregion

#region Params

public abstract class SemioPersistentParam<T> : GH_PersistentParam<T> where T : class, IGH_Goo
{
    protected SemioPersistentParam(string name, string nickname, string description, string category,
        string subcategory) : base(name, nickname, description, category, subcategory)
    {
    }
}

public class RepresentationParam : SemioPersistentParam<RepresentationGoo>
{
    public RepresentationParam() : base("Representation", "Rp", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");

    protected override Bitmap Icon => Resources.representation_24x24;

    protected override GH_GetterResult Prompt_Singular(ref RepresentationGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<RepresentationGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class SpecifierParam : SemioPersistentParam<SpecifierGoo>
{
    public SpecifierParam() : base("Specifier", "Sp", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("DBE104DA-63FA-4C68-8D41-834DD962F1D7");

    protected override Bitmap Icon => Resources.specifier_24x24;

    protected override GH_GetterResult Prompt_Singular(ref SpecifierGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<SpecifierGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class PortParam : SemioPersistentParam<PortGoo>
{
    public PortParam() : base("Port", "Po", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");

    protected override Bitmap Icon => Resources.port_24x24;

    protected override GH_GetterResult Prompt_Singular(ref PortGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<PortGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class QualityParam : SemioPersistentParam<QualityGoo>
{
    public QualityParam() : base("Quality", "Ql", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("F2F6F2F9-7F0E-4F0F-9F0C-7F6F6F6F6F6F");

    protected override Bitmap Icon => Resources.quality_24x24;

    protected override GH_GetterResult Prompt_Singular(ref QualityGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<QualityGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class TypeParam : SemioPersistentParam<TypeGoo>
{
    public TypeParam() : base("Type", "Ty", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");

    protected override Bitmap Icon => Resources.type_24x24;

    protected override GH_GetterResult Prompt_Singular(ref TypeGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<TypeGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class ScreenPointParam : SemioPersistentParam<ScreenPointGoo>
{
    public ScreenPointParam() : base("Screen Point", "SP", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");

    protected override Bitmap Icon => Resources.screenpoint_24x24;

    protected override GH_GetterResult Prompt_Singular(ref ScreenPointGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<ScreenPointGoo> values)
    {
        throw new NotImplementedException();
    }
}
public class PieceParam : SemioPersistentParam<PieceGoo>
{
    public PieceParam() : base("Piece", "Pc", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");

    protected override Bitmap Icon => Resources.piece_24x24;

    protected override GH_GetterResult Prompt_Singular(ref PieceGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<PieceGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class SideParam : SemioPersistentParam<SideGoo>
{
    public SideParam() : base("Side", "Sd", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("4FDE465D-39AB-41C7-AF82-252F1F7C80B9");

    protected override Bitmap Icon => Resources.side_24x24;

    protected override GH_GetterResult Prompt_Singular(ref SideGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<SideGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class AttractionParam : SemioPersistentParam<AttractionGoo>
{
    public AttractionParam() : base("Attraction", "At", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");

    protected override Bitmap Icon => Resources.attraction_24x24;

    protected override GH_GetterResult Prompt_Singular(ref AttractionGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<AttractionGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class FormationParam : SemioPersistentParam<FormationGoo>
{
    public FormationParam() : base("Formation", "Fo", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");

    protected override Bitmap Icon => Resources.formation_24x24;

    protected override GH_GetterResult Prompt_Singular(ref FormationGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<FormationGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class SceneParam : SemioPersistentParam<SceneGoo>
{
    public SceneParam() : base("Scene", "Sn", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("7E26A3C8-4F95-485D-8288-63DC9C44E9A4");

    protected override Bitmap Icon => Resources.scene_24x24;

    protected override GH_GetterResult Prompt_Singular(ref SceneGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<SceneGoo> values)
    {
        throw new NotImplementedException();
    }
}

#endregion

#region Components

public abstract class SemioComponent : GH_Component
{
    public SemioComponent(string name, string nickname, string description, string category, string subcategory) : base(
        name, nickname, description, category, subcategory)
    {
    }
}

#region Modelling

public class RepresentationComponent : SemioComponent
{
    public RepresentationComponent()
        : base("Model Representation", "~Rep",
            "Construct, deconstruct or modify a representation.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

    protected override Bitmap Icon => Resources.representation_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp?",
            "Optional representation to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Url", "Ur", "Url of the representation. Either a relative file path or link.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Level of Detail", "Ld?", "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \nThere can be only one default representation per type.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list,new List<string>());
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp",
            "Constructed or modified representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "Url of the representation. Either a relative file path or link.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("LoD", "Ld?", "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \\nThere can be only one default representation per type.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var representationGoo = new RepresentationGoo();
        var url = "";
        var lod = "";
        var tags = new List<string>();

        DA.GetData(0, ref representationGoo);
        if (DA.GetData(1, ref url))
            representationGoo.Value.Url = url;
        if (DA.GetData(2, ref lod))
            representationGoo.Value.Lod = lod;
        if (DA.GetDataList(3, tags))
            representationGoo.Value.Tags = tags;

        var isValidInput = true;
        if (representationGoo.Value.Url == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A representation needs an url.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, representationGoo.Duplicate());
        DA.SetData(1, representationGoo.Value.Url);
        DA.SetData(2, representationGoo.Value.Lod??"");
        DA.SetDataList(3, representationGoo.Value.Tags);
    }
}

public class SpecifierComponent : SemioComponent
{
    public SpecifierComponent()
        : base("Model Specifier", "~Spc",
            "Construct, deconstruct or modify a specifier.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("2552DB71-8459-4DB5-AD66-723573E771A2");

    protected override Bitmap Icon => Resources.specifier_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SpecifierParam(), "Specifier", "Sp?",
            "Optional specifier to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Context", "Ct", "Context of the specifier.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Group", "Gr", "Group of the specifier.", GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SpecifierParam(), "Specifier", "Sp",
            "Constructed or modified specifier.", GH_ParamAccess.item);
        pManager.AddTextParameter("Context", "Ct", "Context of the specifier.", GH_ParamAccess.item);
        pManager.AddTextParameter("Group", "Gr", "Group of the specifier.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var specifierGoo = new SpecifierGoo();
        var context = "";
        var group = "";

        DA.GetData(0, ref specifierGoo);
        if (DA.GetData(1, ref context))
            specifierGoo.Value.Context = context;
        if (DA.GetData(2, ref group))
            specifierGoo.Value.Group = group;

        var isValidInput = true;
        if (specifierGoo.Value.Context == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A specifier needs a context.");
            isValidInput = false;
        }
        if (specifierGoo.Value.Group == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A specifier needs a group.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, specifierGoo.Duplicate());
        DA.SetData(1, specifierGoo.Value.Context);
        DA.SetData(2, specifierGoo.Value.Group);
    }
}

public class PortComponent : SemioComponent
{
    public PortComponent()
        : base("Model Port", "~Por",
            "Construct, deconstruct or modify a port.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");

    protected override Bitmap Icon => Resources.port_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "Po?",
            "Optional port to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddPlaneParameter("Plane", "Pl", "Plane of the port.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddParameter(new SpecifierParam(), "Specifiers", "Sp+", "Specifiers of the port.",
            GH_ParamAccess.list);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "Po",
            "Constructed or modified port.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pl", "Plane of the port.", GH_ParamAccess.item);
        pManager.AddParameter(new SpecifierParam(), "Specifiers", "Sp+", "Specifiers to identify the port.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var portGoo = new PortGoo();
        var planeGeo = new Rhino.Geometry.Plane();
        var specifierGoos = new List<SpecifierGoo>();

        DA.GetData(0, ref portGoo);
        if (DA.GetData(1, ref planeGeo))
            portGoo.Value.Plane = planeGeo.convert();
        if (DA.GetDataList(2, specifierGoos))
            portGoo.Value.Specifiers = specifierGoos.Select(s => s.Value).ToList();

        var isValidInput = true;
        if (portGoo.Value.Plane == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs a plane.");
            isValidInput = false;
        }

        if (portGoo.Value.Specifiers == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs at least one specifier.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, portGoo.Duplicate());
        DA.SetData(1, portGoo.Value.Plane?.convert());
        DA.SetDataList(2, portGoo.Value.Specifiers?.Select(s => new SpecifierGoo(s.DeepClone()
            ))??new List<SpecifierGoo>());
    }
}

public class QualityComponent : SemioComponent
{
    public QualityComponent()
        : base("Model Quality", "~Qlt",
            "Construct, deconstruct or modify a quality.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");

    protected override Bitmap Icon => Resources.quality_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql?",
            "Optional quality to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "Na", "Name of the quality.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Value", "Va", "Value of the quality.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Unit", "Un?", " Optional unit of the quality.", GH_ParamAccess.item);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
            "Constructed or modified quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va", "Value of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "Optional unit of the quality.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var qualityGoo = new QualityGoo();
        var name = "";
        var value = "";
        var unit = "";

        DA.GetData(0, ref qualityGoo);
        if (DA.GetData(1, ref name))
            qualityGoo.Value.Name = name;
        if (DA.GetData(2, ref value))
            qualityGoo.Value.Value = value;
        if (DA.GetData(3, ref unit))
            qualityGoo.Value.Unit = unit;

        var isValidInput = true;
        if (qualityGoo.Value.Name == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A quality needs a name.");
            isValidInput = false;
        }

        if (qualityGoo.Value.Value == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A quality needs a value.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, qualityGoo.Duplicate());
        DA.SetData(1, qualityGoo.Value.Name);
        DA.SetData(2, qualityGoo.Value.Value);
        DA.SetData(3, qualityGoo.Value.Unit);
    }
}

public class TypeComponent : SemioComponent
{
    public TypeComponent()
        : base("Model Type", "~Typ",
            "Construct, deconstruct or modify a type.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");

    protected override Bitmap Icon => Resources.type_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty?",
            "Optional type to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "Na", "Name of the type.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the type.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the type.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddTextParameter("Variant", "Vn?", "Optional variant of the type.",
                       GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Unit", "Ut", "Unit of the type.", GH_ParamAccess.item);
        pManager[5].Optional = true;
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+", "Representations of the type.",
            GH_ParamAccess.list);
        pManager[6].Optional = true;
        pManager.AddParameter(new PortParam(), "Ports", "Po+", "Ports of the type.", GH_ParamAccess.list);
        pManager[7].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the type. They can be further used to distinguish types with the same name.",
            GH_ParamAccess.list);
        pManager[8].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty",
            "Constructed or modified type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "Vn?", "Optional variant of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "Unit of the type. By default the document unit is used. Otherwise meters will be used.", GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+", "Representations of the",
            GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Ports", "Po+", "Ports of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*", "Optional qualities of the type.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var typeGoo = new TypeGoo();
        var name = "";
        var description = "";
        var icon = "";
        var variant = "";
        var unit = "";
        var representationGoos = new List<RepresentationGoo>();
        var portGoos = new List<PortGoo>();
        var qualityGoos = new List<QualityGoo>();

        DA.GetData(0, ref typeGoo);
        if (DA.GetData(1, ref name))
            typeGoo.Value.Name = name;
        if (DA.GetData(2, ref description))
            typeGoo.Value.Description = description;
        if (DA.GetData(3, ref icon))
            typeGoo.Value.Icon = icon;
        if (DA.GetData(4, ref variant))
            typeGoo.Value.Variant = variant;
        if (!DA.GetData(5, ref unit))
        {
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                typeGoo.Value.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                typeGoo.Value.Unit = "m";
            }

        }
        else
            typeGoo.Value.Unit = unit;
        if (DA.GetDataList(6, representationGoos))
            typeGoo.Value.Representations = representationGoos.Select(r => r.Value).ToList();
        if (DA.GetDataList(7, portGoos))
            typeGoo.Value.Ports = portGoos.Select(p => p.Value).ToList();
        if (DA.GetDataList(8, qualityGoos))
            typeGoo.Value.Qualities = qualityGoos.Select(q => q.Value).ToList();

        var isValidInput = true;
        if (typeGoo.Value.Name == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs a name.");
            isValidInput = false;
        }
        // currently impossible
        if (typeGoo.Value.Unit == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs a unit.");
            isValidInput = false;
        }
        if (!Utility.IsValidUnit(typeGoo.Value.Unit))
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The unit is not valid.");
            isValidInput = false;
        }
        if (typeGoo.Value.Representations == null || typeGoo.Value.Representations.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs at least one representation.");
            isValidInput = false;
        }
        if (typeGoo.Value.Ports == null || typeGoo.Value.Ports.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs at least one port.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, typeGoo.Duplicate());
        DA.SetData(1, typeGoo.Value.Name);
        DA.SetData(2, typeGoo.Value.Description??"");
        DA.SetData(3, typeGoo.Value.Icon??"");
        DA.SetData(4, typeGoo.Value.Variant??"");
        DA.SetData(5, typeGoo.Value.Unit);
        DA.SetDataList(6, typeGoo.Value.Representations.Select(r => new RepresentationGoo(r.DeepClone())));
        DA.SetDataList(7, typeGoo.Value.Ports.Select(p => new PortGoo(p.DeepClone())));
        DA.SetDataList(8, typeGoo.Value.Qualities?.Select(q => new QualityGoo(q.DeepClone()))??new List<QualityGoo>());
    }
}

public class ScreenPointComponent : SemioComponent
{
    public ScreenPointComponent()
        : base("Model Screen Point", "~SP",
            "Construct, deconstruct or modify a screen point.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");

    protected override Bitmap Icon => Resources.screenpoint_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ScreenPointParam(), "Screen Point", "SP?",
                       "Optional screen point to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddIntegerParameter("X", "X", "X coordinate of the screen point.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddIntegerParameter("Y", "Y", "Y coordinate of the screen point.", GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ScreenPointParam(), "Screen Point", "SP",
                       "Constructed or modified screen point.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("X", "X", "X coordinate of the screen point.", GH_ParamAccess.item);
        pManager.AddIntegerParameter("Y", "Y", "Y coordinate of the screen point.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var screenPointGoo = new ScreenPointGoo();
        var x = 0;
        var y = 0;

        DA.GetData(0, ref screenPointGoo);
        if (DA.GetData(1, ref x))
            screenPointGoo.Value.X = x;
        if (DA.GetData(2, ref y))
            screenPointGoo.Value.Y = y;

        var isValidInput = true;
        if (screenPointGoo.Value.X == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A screen point needs an x coordinate.");
            isValidInput = false;
        }
        if (screenPointGoo.Value.Y == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A screen point needs a y coordinate.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, screenPointGoo.Duplicate());
        DA.SetData(1, screenPointGoo.Value.X);
        DA.SetData(2, screenPointGoo.Value.Y);
    }
}

public class PieceComponent : SemioComponent
{
    public PieceComponent()
        : base("Model Piece", "~Pce",
            "Construct, deconstruct or modify a piece.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");

    protected override Bitmap Icon => Resources.piece_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "Pc?",
            "Optional piece to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Id", "Id",
            "Id of the piece. If none is provided then a newly generated one will be produced. \nWARNING: The generated id only works if this component constructs one piece.\nFor multiple pieces use the Random Id component.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Type Name", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Type Variant", "TyVn?", "Optional variant of the type of the piece.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddPlaneParameter("Root Plane", "RtPn?", "Optional root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never attracted.", GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddParameter(new ScreenPointParam(), "Diagram Screen Point", "DgSP", "Screen point of the piece in the diagram.",
            GH_ParamAccess.item);
        pManager[5].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "Pc",
            "Constructed or modified piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id", "Id of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Name", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Variant", "TyVn?", "Optional variant of the type of the piece.", GH_ParamAccess.item);
        pManager.AddPlaneParameter("Root Plane", "RtPl?", "Root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never attracted.", GH_ParamAccess.item);
        pManager.AddParameter(new ScreenPointParam(), "Diagram Screen Point", "DgSP", "Screen point of the piece in the diagram.",
                       GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var pieceGoo = new PieceGoo();
        var id = "";
        var typeName = "";
        var typeVariant = "";
        var rootPlane = new Rhino.Geometry.Plane();
        var screenPointGoo = new ScreenPointGoo();

        var existingPiece = DA.GetData(0, ref pieceGoo);
        if (DA.GetData(1, ref id))
            pieceGoo.Value.Id = id;
        else if (!existingPiece)
                pieceGoo.Value.Id = Generator.GenerateRandomId(BitConverter.ToInt32(InstanceGuid.ToByteArray(), 0));
        if (DA.GetData(2, ref typeName))
        {
            if (pieceGoo.Value.Type == null)
                pieceGoo.Value.Type = new TypeId { Name = typeName };
            else
                pieceGoo.Value.Type.Name = typeName;
        }
        if (DA.GetData(3, ref typeVariant))
            pieceGoo.Value.Type.Variant = typeVariant;
        if (DA.GetData(4, ref rootPlane))
            pieceGoo.Value.Root = new RootPiece { Plane = rootPlane.convert() };
        if (DA.GetData(5, ref screenPointGoo))
            pieceGoo.Value.Diagram = new DiagramPiece() { Point = screenPointGoo.Value };

        var isValidInput = true;
        if (pieceGoo.Value.Id == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A piece needs an id.");
            isValidInput = false;
        }
        if (pieceGoo.Value.Type?.Name == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A piece needs a type.");
            isValidInput = false;
        }
        if (pieceGoo.Value.Diagram?.Point == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A piece needs a screen point in the diagram.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, pieceGoo.Duplicate());
        DA.SetData(1, pieceGoo.Value.Id);
        DA.SetData(2, pieceGoo.Value.Type?.Name);
        DA.SetData(3, pieceGoo.Value.Type.Variant??"");
        DA.SetData(4, pieceGoo.Value.Root?.Plane?.convert());
        DA.SetData(5, new ScreenPointGoo(pieceGoo.Value.Diagram.Point));
    }
}

public class SideComponent : SemioComponent
{
    public SideComponent()
        : base("Model Side", "~Sde",
            "Construct, deconstruct or modify a side.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("AE68EB0B-01D6-458E-870E-346E7C9823B5");

    protected override Bitmap Icon => Resources.side_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SideParam(), "Side", "Sd?",
            "Optional side to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Piece Id", "PcId", "Id of the piece of the side.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddParameter(new SpecifierParam(), "Piece Type Port Specifiers", "PcTyPoSp+",
            "Specifiers to identify the port of the type of the piece of the side.", GH_ParamAccess.list);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SideParam(), "Side", "Sd",
            "Constructed or modified side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Piece Id", "PcId", "Id of the piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new SpecifierParam(), "Piece Type Port Specifiers", "PcTyPoSp+",
            "Specifiers to identify the port of type of the piece of the side.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var sideGoo = new SideGoo();
        var pieceId = "";
        var pieceTypePortSpecifiers = new List<SpecifierGoo>();

        DA.GetData(0, ref sideGoo);
        if (DA.GetData(1, ref pieceId))
            sideGoo.Value.Piece.Id = pieceId;
        if (DA.GetDataList(2, pieceTypePortSpecifiers))
        {
            if (sideGoo.Value.Piece.Type == null)
                sideGoo.Value.Piece.Type = new TypePieceSide();
            if (sideGoo.Value.Piece.Type.Port == null)
                sideGoo.Value.Piece.Type.Port = new PortId();
            sideGoo.Value.Piece.Type.Port.Specifiers = pieceTypePortSpecifiers.Select(s => s.Value).ToList();
        }

        var isValidInput = true;
        if (sideGoo.Value.Piece.Id == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A side needs a piece id.");
            isValidInput = false;
        }

        if (sideGoo.Value.Piece.Type.Port.Specifiers == null || sideGoo.Value.Piece.Type.Port.Specifiers.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A side needs at least one piece type port specifier.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, sideGoo.Duplicate());
        DA.SetData(1, sideGoo.Value.Piece.Id);
        DA.SetDataList(2, sideGoo.Value.Piece.Type.Port.Specifiers.Select(s => new SpecifierGoo(s.DeepClone())));
    }
}

public class AttractionComponent : SemioComponent
{
    public AttractionComponent()
        : base("Model Attraction", "~Atr",
            "Construct, deconstruct or modify an attraction.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");

    protected override Bitmap Icon => Resources.attraction_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new AttractionParam(), "Attraction", "At?",
            "Optional attraction to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddParameter(new SideParam(), "Attracting Side", "AnSd", "Attracting side of the attraction.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddParameter(new SideParam(), "Attracted Side", "AdSd", "Attracted side of the attraction.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new AttractionParam(), "Attraction", "At",
            "Constructed or modified attraction.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Attracting Side", "AnSd", "Attracting side of the attraction.",
            GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Attracted Side", "AdSd", "Attracted side of the attraction.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var attractionGoo = new AttractionGoo();
        var attractingSideGoo = new SideGoo();
        var attractedSideGoo = new SideGoo();

        DA.GetData(0, ref attractionGoo);
        if (DA.GetData(1, ref attractingSideGoo)) attractionGoo.Value.Attracting = attractingSideGoo.Value;
        if (DA.GetData(2, ref attractedSideGoo)) attractionGoo.Value.Attracted = attractedSideGoo.Value;

        DA.SetData(0, attractionGoo.Duplicate());
        if (attractionGoo.Value.Attracting != null)
            DA.SetData(1, new SideGoo(attractionGoo.Value.Attracting.DeepClone()));
        if (attractionGoo.Value.Attracted != null)
            DA.SetData(2, new SideGoo(attractionGoo.Value.Attracted?.DeepClone()));
    }
}

public class FormationComponent : SemioComponent
{
    public FormationComponent()
        : base("Model Formation", "~For",
            "Construct, deconstruct or modify a formation.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");

    protected override Bitmap Icon => Resources.formation_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo?",
            "Optional formation to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "Na", "Name of the formation.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the formation.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the formation.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddTextParameter("Variant", "Vn?", "Optional variant of the formation.", GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Unit", "Ut", "Unit of the formation.", GH_ParamAccess.item);
        pManager[5].Optional = true;
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc+", "Pieces of the formation.", GH_ParamAccess.list);
        pManager[6].Optional = true;
        pManager.AddParameter(new AttractionParam(), "Attractions", "At+", "Attractions of the formation.",
            GH_ParamAccess.list);
        pManager[7].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the formation.",
            GH_ParamAccess.list);
        pManager[8].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo",
            "Constructed or modified formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "Vn?", "Optional variant of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "Unit of the formation.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc+", "Pieces of the formation.", GH_ParamAccess.list);
        pManager.AddParameter(new AttractionParam(), "Attractions", "At+", "Attractions of the formation.",
            GH_ParamAccess.list);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*", "Optional qualities of the formation.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var formationGoo = new FormationGoo();
        var name = "";
        var description = "";
        var icon = "";
        var variant = "";
        var unit = "";
        var pieceGoos = new List<PieceGoo>();
        var attractionGoos = new List<AttractionGoo>();
        var qualityGoos = new List<QualityGoo>();

        DA.GetData(0, ref formationGoo);
        if (DA.GetData(1, ref name))
            formationGoo.Value.Name = name;
        if (DA.GetData(2, ref description))
            formationGoo.Value.Description = description;
        if (DA.GetData(3, ref icon))
            formationGoo.Value.Icon = icon;
        if (DA.GetData(4, ref variant))
            formationGoo.Value.Variant = variant;
        if (!DA.GetData(5, ref unit))
        {
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                formationGoo.Value.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                formationGoo.Value.Unit = "m";
            }

        }
        else
            formationGoo.Value.Unit = unit;
        if (DA.GetDataList(6, pieceGoos))
            formationGoo.Value.Pieces = pieceGoos.Select(p => p.Value).ToList();
        if (DA.GetDataList(7, attractionGoos))
            formationGoo.Value.Attractions = attractionGoos.Select(a => a.Value).ToList();
        if (DA.GetDataList(8, qualityGoos))
            formationGoo.Value.Qualities = qualityGoos.Select(q => q.Value).ToList();

        var isValidInput = true;
        if (formationGoo.Value.Name == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs a name.");
            isValidInput = false;
        }
        // currently impossible
        if (formationGoo.Value.Unit == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs a unit.");
            isValidInput = false;
        }
        if (!Utility.IsValidUnit(formationGoo.Value.Unit))
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The unit is not valid.");
            isValidInput = false;
        }
        if (formationGoo.Value.Pieces == null || formationGoo.Value.Pieces.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs at least one piece.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, formationGoo.Duplicate());
        DA.SetData(1, formationGoo.Value.Name);
        DA.SetData(2, formationGoo.Value.Description??"");
        DA.SetData(3, formationGoo.Value.Icon??"");
        DA.SetData(4, formationGoo.Value.Variant??"");
        DA.SetData(5, formationGoo.Value.Unit);
        DA.SetDataList(6, formationGoo.Value.Pieces.Select(p => new PieceGoo(p.DeepClone())));
        DA.SetDataList(7, formationGoo.Value.Attractions.Select(a => new AttractionGoo(a.DeepClone())));
        DA.SetDataList(8, formationGoo.Value.Qualities?.Select(q => new QualityGoo(q.DeepClone()))??new List<QualityGoo>());
    }
}

public class RandomIdsComponent : SemioComponent
{
    public RandomIdsComponent()
        : base("Random Ids", "%Ids",
            "Generate random ids.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("27E48D59-10BE-4239-8AAC-9031BF6AFBCC");

    protected override Bitmap Icon => Resources.id_random_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddIntegerParameter("Count", "Ct", "Number of ids to generate.", GH_ParamAccess.item, 1);
        pManager.AddIntegerParameter("Seed", "Se", "Seed for the random generator.", GH_ParamAccess.item, 0);
        pManager.AddBooleanParameter("Unique Component", "UC",
            "If true, the generated ids will be unique for this component.", GH_ParamAccess.item, true);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Ids", "Id+", "Generated ids.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var count = 0;
        var seed = 0;
        var unique = true;

        DA.GetData(0, ref count);
        DA.GetData(1, ref seed);
        DA.GetData(2, ref unique);

        var ids = new List<string>();

        for (var i = 0; i < count; i++)
        {
            var hashString = seed + ";" + i;
            if (unique)
                hashString += ";" + InstanceGuid;
            using (var md5 = MD5.Create())
            {
                var hash = md5.ComputeHash(Encoding.UTF8.GetBytes(hashString));
                var id = Generator.GenerateRandomId(BitConverter.ToInt32(hash, 0));
                ids.Add(id);
            }
        }

        DA.SetDataList(0, ids);
    }
}

#endregion

#region Loading/Saving

public class LoadKitComponent : SemioComponent
{
    public LoadKitComponent()
        : base("Load Kit", "/Kit",
            "Load a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");

    protected override Bitmap Icon => Resources.kit_load_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Load the kit.", GH_ParamAccess.item, false);
    }


    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "Na", "Name of the kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur?", "Optional url of the kit", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "Optional types of the kit", GH_ParamAccess.list);
        pManager.AddParameter(new FormationParam(), "Formations", "Fo*", "Optional formations of the kit",
            GH_ParamAccess.list);
    }


    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var path = "";
        var run = false;

        if (!DA.GetData(0, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();

        DA.GetData(1, ref run);

        if (run)
        {
            var response = new Api().LoadLocalKit(path);
            if (response == null)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
                return;
            }

            if (response.Error != null)
            {
                AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error);
                return;
            }

            var kit = response.Kit;

            DA.SetData(0, kit.Name);
            DA.SetData(1, kit.Description);
            DA.SetData(2, kit.Icon);
            DA.SetData(3, kit.Url);
            DA.SetDataList(4, kit.Types?.Select(t => new TypeGoo(t.DeepClone())));
            DA.SetDataList(5, kit.Formations?.Select(f => new FormationGoo(f.DeepClone())));
        }
    }
}

public class CreateKitComponent : SemioComponent
{
    public CreateKitComponent()
        : base("Create Kit", "+Kit",
            "Create a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("1CC1BE06-85B8-4B0E-A59A-35B4D7C6E0FD");

    protected override Bitmap Icon => Resources.kit_create_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "Na", "Name of the kit", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the kit", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the kit", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Url", "Ur?", "Optional url of the kit", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "Optional types of the kit", GH_ParamAccess.list);
        pManager[4].Optional = true;
        pManager.AddParameter(new FormationParam(), "Formations", "Fo*", "Optional formations of the kit",
            GH_ParamAccess.list);
        pManager[5].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will take the current directory from which the Grasshopper script is executed.",
            GH_ParamAccess.item);
        pManager[6].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Create the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the kit was created.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var name = "";
        var description = "";
        var icon = "";
        var url = "";
        var typeGoos = new List<TypeGoo>();
        var formationGoos = new List<FormationGoo>();
        var path = "";
        var run = false;

        DA.GetData(0, ref name);
        DA.GetData(1, ref description);
        DA.GetData(2, ref icon);
        DA.GetData(3, ref url);
        DA.GetDataList(4, typeGoos);
        DA.GetDataList(5, formationGoos);
        if (!DA.GetData(6, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(7, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var kit = new Kit
        {
            Name = name,
            Description = description,
            Icon = icon,
            Url = url,
            Types = typeGoos.Select(t => t.Value).ToList(),
            Formations = formationGoos.Select(f => f.Value).ToList()
        };

        var response = new Api().CreateLocalKit(path, kit);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }

        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            DA.SetData(0, false);
            return;
        }

        DA.SetData(0, true);
    }
}

public class DeleteKitComponent : SemioComponent
{
    public DeleteKitComponent()
        : base("Delete Kit", "-Kit",
            "Delete a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("38D4283C-510C-4E77-9105-92A5BE3E3BA0");

    protected override Bitmap Icon => Resources.kit_delete_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Delete the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the kit was deleted.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var path = "";
        var run = false;

        if (!DA.GetData(0, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(1, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var response = new Api().DeleteLocalKit(path);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.ToString());
            DA.SetData(0, false);
            return;
        }

        DA.SetData(0, true);
    }
}

#region Adding

public class AddTypeComponent : SemioComponent
{
    public AddTypeComponent()
        : base("Add Type", "+Typ",
            "Add a type to a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("BC46DC07-C0BE-433F-9E2F-60CCBAA39148");

    protected override Bitmap Icon => Resources.type_add_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty",
            "Type to add to the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Add the type to the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the type was added to the kit.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var typeGoo = new TypeGoo();
        var path = "";
        var run = false;

        DA.GetData(0, ref typeGoo);
        if (!DA.GetData(1, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(2, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var response = new Api().AddTypeToLocalKit(path, typeGoo.Value);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }
  
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            DA.SetData(0, false);
            return;
        }

        DA.SetData(0, true);
    }
}

public class AddFormationComponent : SemioComponent
{
    public AddFormationComponent()
        : base("Add Formation", "+For",
            "Add a formation to a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("8B7AA946-0CB1-4CA8-A712-610B60425368");

    protected override Bitmap Icon => Resources.formation_add_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo",
            "Formation to add to the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Add the formation to the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the formation was added to the kit.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var formationGoo = new FormationGoo();
        var path = "";
        var run = false;

        DA.GetData(0, ref formationGoo);
        if (!DA.GetData(1, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(2, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var response = new Api().AddFormationToLocalKit(path, formationGoo.Value);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            DA.SetData(0, false);
            return;
        }

        DA.SetData(0, true);
        
    }
}

#endregion

#region Removing

public class RemoveTypeComponent : SemioComponent
{
    public RemoveTypeComponent()
        : base("Remove Type", "-Typ",
            "Remove a type from a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("F38D0E82-5A58-425A-B705-7A62FD9DB957");

    protected override Bitmap Icon => Resources.type_remove_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Type Name", "TyNa",
            "Name of the type to remove from the kit.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Type Qualities", "TyQl*",
            "If there is more than one type with the same name use qualities to precisely identify the type.",
            GH_ParamAccess.list);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Remove the type from the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the type was removed from the kit.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var typeName = "";
        var typeQualityGoos = new List<QualityGoo>();
        var path = "";
        var run = false;

        DA.GetData(0, ref typeName);
        DA.GetDataList(1, typeQualityGoos);
        if (!DA.GetData(2, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(3, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var type = new TypeId
        {
            Name = typeName
        };
        //if (typeQualityGoos.Count > 0) type.Qualities = typeQualityGoos.Select(q => q.Value).ToList();
        var response = new Api().RemoveTypeFromLocalKit(path, type);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            DA.SetData(0, false);
            return;
        }
 
        DA.SetData(0, true);
    }
}

public class RemoveFormationComponent : SemioComponent
{
    public RemoveFormationComponent()
        : base("Remove Formation", "-For",
            "Remove a formation from a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("9ECCE095-9D1E-4554-A3EB-1EAEEE2B12D5");

    protected override Bitmap Icon => Resources.formation_remove_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Formation Name", "FoNa",
            "Name of the formation to remove from the kit.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "Formation Qualities", "FoQl*",
            "If there is more than one formation with the same name use qualities to precisely identify the formation.",
            GH_ParamAccess.list);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Remove the formation from the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the formation was removed from the kit.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var formationName = "";
        var formationQualityGoos = new List<QualityGoo>();
        var path = "";
        var run = false;

        DA.GetData(0, ref formationName);
        DA.GetDataList(1, formationQualityGoos);
        if (!DA.GetData(2, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(3, ref run);

        if (!run)
        {
            DA.SetData(0, false);
            return;
        }

        var formation = new FormationId
        {
            Name = formationName
        };
        //if (formationQualityGoos.Count > 0) formation.Qualities = formationQualityGoos.Select(q => q.Value).ToList();
        var response = new Api().RemoveFormationFromLocalKit(path, formation);
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            DA.SetData(0, false);
            return;
        }
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            DA.SetData(0, false);
            return;
        }
        DA.SetData(0, true);
    }
}

#endregion

#endregion

#region Scripting

public class EncodeTextComponent : SemioComponent
{
    public EncodeTextComponent()
        : base("Encode Text", ">Txt",
            "Encode a text.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("FBDDF723-80BD-4AF9-A1EE-450A27D50ABE");

    protected override Bitmap Icon => Resources.encode_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text to encode.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "EnTx", "Encoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";

        DA.GetData(0, ref text);

        var textBytes = Encoding.UTF8.GetBytes(text);
        var base64Text = Convert.ToBase64String(textBytes);

        DA.SetData(0, base64Text);
    }
}

public class DecodeTextComponent : SemioComponent
{
    public DecodeTextComponent()
        : base("Decode Text", "<Txt",
            "Decode a text.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("E7158D28-87DE-493F-8D78-923265C3E211");

    protected override Bitmap Icon => Resources.decode_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "EnTx", "Encoded text to decode.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Decoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);

        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        DA.SetData(0, text);
    }
}

#region Serialize

public class SerializeQualityComponent : SemioComponent
{
    public SerializeQualityComponent()
        : base("Serialize Quality", ">Qlt",
            "Serialize a quality.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");

    protected override Bitmap Icon => Resources.quality_serialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
            "Quality to serialize.", GH_ParamAccess.item);
        pManager.AddBooleanParameter("Run", "R", "Serialize the quality.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized quality.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var qualityGoo = new QualityGoo();

        DA.GetData(0, ref qualityGoo);

        var text = qualityGoo.Value.Serialize();
        var textBytes = Encoding.UTF8.GetBytes(text);
        var base64Text = Convert.ToBase64String(textBytes);

        DA.SetData(0, base64Text);
    }
}

public class SerializeTypeComponent : SemioComponent
{
    public SerializeTypeComponent()
        : base("Serialize Type", ">Typ",
            "Serialize a type.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");

    protected override Bitmap Icon => Resources.type_serialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty",
            "Type to serialize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized type.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var typeGoo = new TypeGoo();

        DA.GetData(0, ref typeGoo);
        var text = typeGoo.Value.Serialize();
        var textBytes = Encoding.UTF8.GetBytes(text);
        var base64Text = Convert.ToBase64String(textBytes);

        DA.SetData(0, base64Text);
    }
}

public class SerializeFormationComponent : SemioComponent
{
    public SerializeFormationComponent()
        : base("Serialize Formation", ">For",
            "Serialize a formation.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");

    protected override Bitmap Icon => Resources.formation_serialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo",
            "Formation to serialize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized formation.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var formationGoo = new FormationGoo();

        DA.GetData(0, ref formationGoo);
        var text = formationGoo.Value.Serialize();
        var textBytes = Encoding.UTF8.GetBytes(text);
        var base64Text = Convert.ToBase64String(textBytes);

        DA.SetData(0, base64Text);
    }
}

public class SerializeSceneComponent : SemioComponent
{
    public SerializeSceneComponent()
        : base("Serialize Scene", ">Scn",
            "Serialize a scene.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("2470CB4D-FC4A-4DCE-92BF-EDA281B36609");

    protected override Bitmap Icon => Resources.scene_serialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
            "Scene to serialize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized scene.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var sceneGoo = new SceneGoo();

        DA.GetData(0, ref sceneGoo);
        var text = sceneGoo.Value.Serialize();
        var textBytes = Encoding.UTF8.GetBytes(text);
        var base64Text = Convert.ToBase64String(textBytes);

        DA.SetData(0, base64Text);
    }
}

#endregion

#region Deserialize

public class DeserializeQualityComponent : SemioComponent
{
    public DeserializeQualityComponent()
        : base("Deserialize Quality", "<Qlt",
            "Deserialize a quality.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("AECB1169-EB65-470F-966E-D491EB46A625");

    protected override Bitmap Icon => Resources.quality_deserialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized quality.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
            "Deserialized quality.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);

        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        var quality = text.Deserialize<Quality>();

        DA.SetData(0, new QualityGoo(quality));
    }
}

public class DeserializeTypeComponent : SemioComponent
{
    public DeserializeTypeComponent()
        : base("Deserialize Type", "<Typ",
            "Deserialize a type.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("F21A80E0-2A62-4BFD-BC2B-A04363732F84");

    protected override Bitmap Icon => Resources.type_deserialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized type.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty",
            "Deserialized type.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);
        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        var type = text.Deserialize<Type>();

        DA.SetData(0, new TypeGoo(type));
    }
}

public class DeserializeFormationComponent : SemioComponent
{
    public DeserializeFormationComponent()
        : base("Deserialize Formation", "<For",
            "Deserialize a formation.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("464D4D72-CFF1-4391-8C31-9E37EB9434C6");

    protected override Bitmap Icon => Resources.formation_deserialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized formation.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo",
            "Deserialized formation.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);
        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        var formation = text.Deserialize<Formation>();

        DA.SetData(0, new FormationGoo(formation));
    }
}

public class DeserializeSceneComponent : SemioComponent
{
    public DeserializeSceneComponent()
        : base("Deserialize Scene", "<Scn",
            "Deserialize a scene.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("9A9AF239-6019-43E6-A3E1-59838BD5400B");

    protected override Bitmap Icon => Resources.scene_deserialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized scene.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
            "Deserialized scene.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);
        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        var scene = text.Deserialize<Scene>();

        DA.SetData(0, new SceneGoo(scene));
    }
}

#endregion

#endregion

#region Viewing

public class GetSceneComponent : SemioComponent
{
    public GetSceneComponent()
        : base("GetScene", "!Scn",
            "Get a scene from a formation.",
            "semio", "Viewing")
    {
    }

    public override Guid ComponentGuid => new("55F3BF32-3B4D-4355-BFAD-F3CA3847FC94");

    protected override Bitmap Icon => Resources.scene_get_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Formation Name", "FoNa",
            "Name of formation to convert to a scene.", GH_ParamAccess.item);
        pManager.AddTextParameter("Formation Qualities", "FoQl*", "Optional qualities to identify the formation.",
            GH_ParamAccess.list);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Convert the formation to a scene.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SceneParam(), "Scene", "Sc", "Scene.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var formationName = "";
        var formationQualityGoos = new List<QualityGoo>();
        var path = "";
        var run = false;

        DA.GetData(0, ref formationName);
        DA.GetDataList(1, formationQualityGoos);
        if (!DA.GetData(2, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(3, ref run);

        if (!run) return;

        var response = new Api().SceneFromFormationFromLocalKit(path, new FormationId
        {
            Name = formationName,
            //Qualities = formationQualityGoos.Select(q => q.Value).ToList()
        });
        if (response == null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
            return;
        }
        if (response.Error != null)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
            return;
        }

        DA.SetData(0, new SceneGoo(response.Scene));
    }
}

public class FilterSceneComponent : SemioComponent
{
    public FilterSceneComponent()
        : base("FilterScene", "|Scn",
            "Filter a scene.",
            "semio", "Viewing")
    {
    }

    public override Guid ComponentGuid => new("232796C0-5ADF-47FF-9FC4-058CB7003C5A");

    protected override Bitmap Icon => Resources.scene_filter_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
            "Scene to filter.", GH_ParamAccess.item);
        pManager.AddTextParameter("Level of Details", "LD*",
            "Optional level of details of the representations in the scene.",
            GH_ParamAccess.list);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Tags", "Ta*", "Optional tags of the representations in the scene.",
            GH_ParamAccess.list);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Formats", "Ft*", "Optional formats of the representations in the scene.",
            GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+",
            "Representation of the objects of the scene.", GH_ParamAccess.list);
        pManager.AddPlaneParameter("Planes", "Pl+", "Planes of the objects of the scene.", GH_ParamAccess.list);
        pManager.AddTextParameter("Pieces Ids", "PcId+",
            "Ids of the pieces from the formation that correspond to the objects of the scene.", GH_ParamAccess.list);
        pManager.AddTextParameter("Parents Pieces Ids", "PaPcId+",
            "Ids of the parent pieces from the formation that correspond to the objects of the scene.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var sceneGoo = new SceneGoo();
        var lods = new List<string>();
        var tags = new List<string>();
        var formats = new List<string>();

        DA.GetData(0, ref sceneGoo);
        DA.GetDataList(1, lods);
        DA.GetDataList(2, tags);
        DA.GetDataList(3, formats);

        // filter the representations of the scene
        // if lods are used, only the representations with the specified lods are returned
        // if tags are used, each representations must have at least one of the specified tags
        // if formats are used, only the representations with the specified formats are returned
        var representations = sceneGoo.Value.Objects
            .Select(o => o.Piece.Type.Representations
                .First(r =>
                {
                    if (lods.Count > 0)
                        if (!lods.Contains(r.Lod))
                            return false;
                    if (tags.Count > 0)
                    {
                        if (r.Tags == null)
                            return false;
                        if (!r.Tags.Any(t => tags.Contains(t)))
                            return false;
                    }

                    if (formats.Count > 0)
                        if (!formats.Contains(Path.GetExtension(r.Url)))
                            return false;
                    return true;
                })).ToList();
        var planes = sceneGoo.Value.Objects.Select(o => o.Plane).ToList();
        var piecesIds = sceneGoo.Value.Objects.Select(o => o.Piece.Id).ToList();
        var parentsPiecesIds = sceneGoo.Value.Objects.Select(o => o.Parent?.Piece?.Id).ToList();

        DA.SetDataList(0, representations.Select(r => new RepresentationGoo(r.DeepClone())));
        DA.SetDataList(1, planes.Select(p => p.convert()));
        DA.SetDataList(2, piecesIds);
        DA.SetDataList(3, parentsPiecesIds);
    }
}

#endregion

#endregion