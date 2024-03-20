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

namespace Semio.Grasshopper;

// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();
// TODO: IsInvalid is used to check null state which is not clean.
// Think of a better way to handle this.

#region Copilot
//public interface IDeepCloneable<T>
//{
//    T DeepClone();
//}

//public interface IEntity
//{
//    string ToString();
//    bool IsInvalid();
//}

//public class Representation : IDeepCloneable<Representation>, IEntity
//{
//    public Representation()
//    {
//        Url = "";
//        Lod = "";
//        Tags = new List<string>();
//    }

//    public string Url { get; set; }
//    public string Lod { get; set; }
//    public List<string> Tags { get; set; }

//    public Representation DeepClone()
//    {
//        return new Representation
//        {
//            Url = Url,
//            Lod = Lod,
//            Tags = new List<string>(Tags)
//        };
//    }

//    public override string ToString()
//    {
//        return $"Representation(Url: {Url})";
//    }

//    public bool IsInvalid()
//    {
//        return Url == "";
//    }
//}

//public class Specifier : IDeepCloneable<Specifier>, IEntity
//{
//    public Specifier()
//    {
//        Context = "";
//        Group = "";
//    }

//    public string Context { get; set; }
//    public string Group { get; set; }

//    public Specifier DeepClone()
//    {
//        return new Specifier
//        {
//            Context = Context,
//            Group = Group
//        };
//    }

//    public override string ToString()
//    {
//        return $"Specifier(Context: {Context})";
//    }

//    public bool IsInvalid()
//    {
//        return Context == "";
//    }
//}

//public class ScreenPoint : IDeepCloneable<ScreenPoint>, IEntity
//{
//    public ScreenPoint()
//    {
//        X = 0;
//        Y = 0;
//    }

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

//    public bool IsInvalid()
//    {
//        return false;
//    }

//    public bool IsZero()
//    {
//        return X == 0 && Y == 0;
//    }
//}

//public class Point : IDeepCloneable<Point>, IEntity
//{
//    public Point()
//    {
//        X = 0;
//        Y = 0;
//        Z = 0;
//    }

//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//    public Point DeepClone()
//    {
//        return new Point
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

//    public bool IsInvalid()
//    {
//        return false;
//    }

//    public bool IsZero()
//    {
//        return X == 0 && Y == 0 && Z == 0;
//    }
//}

//public class Vector : IDeepCloneable<Vector>, IEntity
//{
//    public Vector()
//    {
//        X = 0;
//        Y = 0;
//        Z = 0;
//    }

//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//    public Vector DeepClone()
//    {
//        return new Vector
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

//    public bool IsInvalid()
//    {
//        return false;
//    }

//    public bool IsZero()
//    {
//        return X == 0 && Y == 0 && Z == 0;
//    }
//}

//public class Plane : IDeepCloneable<Plane>, IEntity
//{
//    public Plane()
//    {
//        Origin = new Point();
//        XAxis = new Vector();
//        YAxis = new Vector();
//    }

//    public Point Origin { get; set; }
//    public Vector XAxis { get; set; }
//    public Vector YAxis { get; set; }

//    public Plane DeepClone()
//    {
//        return new Plane
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

//    public bool IsInvalid()
//    {
//        // TODO: Check if axes are normalized and orthogonal.
//        return Origin.IsZero() && XAxis.IsZero() && YAxis.IsZero();
//    }
//}

//public class Port : IDeepCloneable<Port>, IEntity
//{
//    public Port()
//    {
//        Plane = new Plane();
//        Specifiers = new List<Specifier>();
//    }

//    public Plane Plane { get; set; }
//    public List<Specifier> Specifiers { get; set; }

//    public Port DeepClone()
//    {
//        return new Port
//        {
//            Plane = Plane?.DeepClone(),
//            Specifiers = new List<Specifier>(Specifiers.Select(s => s.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Port({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Plane.IsInvalid() || Specifiers.Any(s => s.IsInvalid());
//    }
//}

//public class PortId : IDeepCloneable<PortId>, IEntity
//{
//    public PortId()
//    {
//        Specifiers = new List<Specifier>();
//    }

//    public List<Specifier> Specifiers { get; set; }

//    public PortId DeepClone()
//    {
//        return new PortId
//        {
//            Specifiers = new List<Specifier>(Specifiers.Select(s => s.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"PortId({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Specifiers.Any(s => s.IsInvalid());
//    }
//}


//public class Quality : IDeepCloneable<Quality>, IEntity
//{
//    public Quality()
//    {
//        Name = "";
//        Value = "";
//        Unit = "";
//    }

//    public string Name { get; set; }
//    public string Value { get; set; }
//    public string Unit { get; set; }

//    public Quality DeepClone()
//    {
//        return new Quality
//        {
//            Name = Name,
//            Value = Value,
//            Unit = Unit
//        };
//    }

//    public override string ToString()
//    {
//        return $"Quality(Name: {Name})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "";
//    }
//}

//public class Type : IDeepCloneable<Type>, IEntity
//{
//    public Type()
//    {
//        Name = "";
//        Description = "";
//        Icon = "";
//        Variant = "";
//        Unit = "";
//        Representations = new List<Representation>();
//        Ports = new List<Port>();
//        Qualities = new List<Quality>();
//    }

//    public string Name { get; set; }
//    public string Description { get; set; }
//    public string Icon { get; set; }
//    public string Variant { get; set; }
//    public string Unit { get; set; }
//    public List<Representation> Representations { get; set; }
//    public List<Port> Ports { get; set; }
//    public List<Quality> Qualities { get; set; }

//    public Type DeepClone()
//    {
//        return new Type
//        {
//            Name = Name,
//            Description = Description,
//            Icon = Icon,
//            Variant = Variant,
//            Unit = Unit,
//            Representations = new List<Representation>(Representations.Select(r => r.DeepClone())),
//            Ports = new List<Port>(Ports.Select(p => p.DeepClone())),
//            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Type(Name: {Name}, Variant: {Variant})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "" || Unit == "" || Representations.Any(r => r.IsInvalid()) || Ports.Any(p => p.IsInvalid()) ||
//               Qualities.Any(q => q.IsInvalid());
//    }
//}

//public class TypeId : IDeepCloneable<TypeId>, IEntity
//{
//    public TypeId()
//    {
//        Name = "";
//        Variant = "";
//    }

//    public string Name { get; set; }
//    public string Variant { get; set; }

//    public TypeId DeepClone()
//    {
//        return new TypeId
//        {
//            Name = Name,
//            Variant = Variant
//        };
//    }

//    public override string ToString()
//    {
//        return $"TypeId(Name: {Name}, Variant: {Variant})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "";
//    }
//}

//public class RootPiece : IDeepCloneable<RootPiece>, IEntity
//{
//    public RootPiece()
//    {
//        Plane = new Plane();
//    }

//    public Plane Plane { get; set; }

//    public RootPiece DeepClone()
//    {
//        return new RootPiece
//        {
//            Plane = Plane.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"RootPiece({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Plane.IsInvalid();
//    }
//}

//public class DiagramPiece : IDeepCloneable<DiagramPiece>, IEntity
//{
//    public DiagramPiece()
//    {
//        Point = new ScreenPoint();
//    }

//    public ScreenPoint Point { get; set; }

//    public DiagramPiece DeepClone()
//    {
//        return new DiagramPiece
//        {
//            Point = Point.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"DiagramPiece({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Point.IsInvalid();
//    }
//}

//public class Piece : IDeepCloneable<Piece>, IEntity
//{
//    public Piece()
//    {
//        Id = "";
//        Type = new TypeId();
//        Root = null;
//        Diagram = new DiagramPiece();
//    }

//    public string Id { get; set; }
//    public TypeId Type { get; set; }
//    public RootPiece? Root { get; set; }
//    public DiagramPiece Diagram { get; set; }

//    public Piece DeepClone()
//    {
//        return new Piece
//        {
//            Id = Id,
//            Type = Type.DeepClone(),
//            Root = Root?.DeepClone(),
//            Diagram = Diagram.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Piece(Id: {Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "" || Type.IsInvalid() || (Root?.IsInvalid() ?? false) || Diagram.IsInvalid();
//    }
//}

//public class PieceId : IDeepCloneable<PieceId>, IEntity
//{
//    public PieceId()
//    {
//        Id = "";
//    }

//    public string Id { get; set; }

//    public PieceId DeepClone()
//    {
//        return new PieceId
//        {
//            Id = Id
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceId(Id: {Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "";
//    }
//}

//public class TypePieceSide : IDeepCloneable<TypePieceSide>, IEntity
//{
//    public TypePieceSide()
//    {
//        Port = new PortId();
//    }

//    public PortId Port { get; set; }

//    public TypePieceSide DeepClone()
//    {
//        return new TypePieceSide
//        {
//            Port = Port.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"TypePieceSide({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Port.IsInvalid();
//    }
//}

//public class PieceSide : IDeepCloneable<PieceSide>, IEntity
//{
//    public PieceSide()
//    {
//        Id = "";
//        Type = new TypePieceSide();
//    }

//    public string Id { get; set; }
//    public TypePieceSide Type { get; set; }

//    public PieceSide DeepClone()
//    {
//        return new PieceSide
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceSide(Id: {Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "" || Type.IsInvalid();
//    }
//}

//public class Side : IDeepCloneable<Side>, IEntity
//{
//    public Side()
//    {
//        Piece = new PieceSide();
//    }

//    public PieceSide Piece { get; set; }

//    public Side DeepClone()
//    {
//        return new Side
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Side({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Piece.IsInvalid();
//    }
//}


//public class Attraction : IDeepCloneable<Attraction>, IEntity
//{
//    public Attraction()
//    {
//        Attracting = new Side();
//        Attracted = new Side();
//    }

//    public Side Attracting { get; set; }
//    public Side Attracted { get; set; }

//    public Attraction DeepClone()
//    {
//        return new Attraction
//        {
//            Attracting = Attracting.DeepClone(),
//            Attracted = Attracted.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Attraction(Attracting(Piece: {Attracting.Piece.Id}), Attracted(Piece: {Attracted.Piece.Id}))";
//    }

//    public bool IsInvalid()
//    {
//        return Attracting.IsInvalid() || Attracted.IsInvalid() || Attracting.Piece.Id == Attracted.Piece.Id;
//    }
//}

//public class Formation : IDeepCloneable<Formation>, IEntity
//{
//    public Formation()
//    {
//        Name = "";
//        Description = "";
//        Icon = "";
//        Variant = "";
//        Unit = "";
//        Pieces = new List<Piece>();
//        Attractions = new List<Attraction>();
//        Qualities = new List<Quality>();
//    }

//    public string Name { get; set; }
//    public string Description { get; set; }
//    public string Icon { get; set; }
//    public string Variant { get; set; }
//    public string Unit { get; set; }
//    public List<Piece> Pieces { get; set; }
//    public List<Attraction> Attractions { get; set; }
//    public List<Quality> Qualities { get; set; }

//    public Formation DeepClone()
//    {
//        return new Formation
//        {
//            Name = Name,
//            Description = Description,
//            Icon = Icon,
//            Variant = Variant,
//            Unit = Unit,
//            Pieces = new List<Piece>(Pieces.Select(p => p.DeepClone())),
//            Attractions = new List<Attraction>(Attractions.Select(a => a.DeepClone())),
//            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Formation(Name: {Name}, Variant: {Variant})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "" || Unit == "" || Pieces.Any(p => p.IsInvalid()) || Attractions.Any(a => a.IsInvalid()) ||
//               Qualities.Any(q => q.IsInvalid());
//    }
//}

//public class FormationId : IDeepCloneable<FormationId>, IEntity
//{
//    public FormationId()
//    {
//        Name = "";
//        Variant = "";
//    }

//    public string Name { get; set; }
//    public string Variant { get; set; }

//    public FormationId DeepClone()
//    {
//        return new FormationId
//        {
//            Name = Name,
//            Variant = Variant
//        };
//    }

//    public override string ToString()
//    {
//        return $"FormationId(Name: {Name}, Variant: {Variant})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "";
//    }
//}

//public class TypePieceObject : IDeepCloneable<TypePieceObject>, IEntity
//{
//    public TypePieceObject()
//    {
//        Representations = new List<Representation>();
//    }

//    public List<Representation> Representations { get; set; }

//    public TypePieceObject DeepClone()
//    {
//        return new TypePieceObject
//        {
//            Representations = new List<Representation>(Representations.Select(f => f.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"TypePieceObject({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Representations.Any(r => r.IsInvalid());
//    }
//}

//public class PieceObject : IDeepCloneable<PieceObject>, IEntity
//{
//    public PieceObject()
//    {
//        Id = "";
//        Type = new TypePieceObject();
//    }

//    public string Id { get; set; }
//    public TypePieceObject Type { get; set; }

//    public PieceObject DeepClone()
//    {
//        return new PieceObject
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"PieceObject(Id: {Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "" || Type.IsInvalid();
//    }
//}


//public class ParentObject : IDeepCloneable<ParentObject>, IEntity
//{
//    public ParentObject()
//    {
//        Piece = new PieceId();
//    }

//    public PieceId Piece { get; set; }

//    public ParentObject DeepClone()
//    {
//        return new ParentObject
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"ParentObject({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Piece.IsInvalid();
//    }
//}

//public class Object : IDeepCloneable<Object>, IEntity
//{
//    public Object()
//    {
//        Piece = new PieceObject();
//        Plane = new Plane();
//        Parent = null;
//    }

//    public PieceObject Piece { get; set; }
//    public Plane Plane { get; set; }
//    public ParentObject? Parent { get; set; }

//    public Object DeepClone()
//    {
//        return new Object
//        {
//            Piece = Piece.DeepClone(),
//            Plane = Plane.DeepClone(),
//            Parent = Parent?.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Object({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Piece.IsInvalid() || Plane.IsInvalid() || (Parent?.IsInvalid() ?? false);
//    }
//}

//public class Scene : IDeepCloneable<Scene>, IEntity
//{
//    public Scene()
//    {
//        Objects = new List<Object>();
//    }

//    public List<Object> Objects { get; set; }

//    public Scene DeepClone()
//    {
//        return new Scene
//        {
//            Objects = new List<Object>(Objects.Select(o => o.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Scene({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Objects.Any(o => o.IsInvalid());
//    }
//}

//public class Kit : IDeepCloneable<Kit>, IEntity
//{
//    public Kit()
//    {
//        Name = "";
//        Description = "";
//        Icon = "";
//        Url = "";
//        Types = new List<Type>();
//        Formations = new List<Formation>();
//    }

//    public string Name { get; set; }
//    public string Description { get; set; }
//    public string Icon { get; set; }
//    public string Url { get; set; }
//    public List<Type> Types { get; set; }
//    public List<Formation> Formations { get; set; }

//    public Kit DeepClone()
//    {
//        return new Kit
//        {
//            Name = Name,
//            Description = Description,
//            Icon = Icon,
//            Url = Url,
//            Types = new List<Type>(Types.Select(t => t.DeepClone())),
//            Formations = new List<Formation>(Formations.Select(f => f.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Kit(Name: {Name}, {GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "" || Types.Any(t => t.IsInvalid()) || Formations.Any(f => f.IsInvalid());
//    }
//}

//public class KitMetadata : IDeepCloneable<KitMetadata>, IEntity
//{
//    public string? Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Url { get; set; }

//    public KitMetadata DeepClone()
//    {
//        var kitMetadata = new KitMetadata();
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

//    public bool IsInvalid()
//    {
//        return false;
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
        return new[] { "nm", "mm", "cm", "dm", "m", "km", "µin", "in", "ft", "yd" }.Contains(unit);
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

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Context);
            target = (Q)ptr;
            return true;
        }

        return false;
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;

        string str = null;
        if (GH_Convert.ToString(source, out str, GH_Conversion.Both))
        {
            Value = new Specifier
            {
                Context = str
            };
            return true;
        }

        return false;
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

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Name);
            target = (Q)ptr;
            return true;
        }

        return false;
    }

    public override bool CastFrom(object source)
    {
        if (source == null) return false;

        string str;
        if (GH_Convert.ToString(source, out str, GH_Conversion.Both))
        {
            Value = new Quality
            {
                Name = str
            };
            return true;
        }

        return false;
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
        pManager.AddTextParameter("Level of Detail", "Ld?",
            "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \nThere can be only one default representation per type.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list,
            new List<string>());
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp",
            "Constructed or modified representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "Url of the representation. Either a relative file path or link.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("LoD", "Ld?",
            "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \\nThere can be only one default representation per type.",
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
        if (representationGoo.Value.Url == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A representation needs an url.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, representationGoo.Duplicate());
        DA.SetData(1, representationGoo.Value.Url);
        DA.SetData(2, representationGoo.Value.Lod);
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
        pManager.AddTextParameter("Group", "Gr?", "Optional group of the specifier. No group means true.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SpecifierParam(), "Specifier", "Sp",
            "Constructed or modified specifier.", GH_ParamAccess.item);
        pManager.AddTextParameter("Context", "Ct", "Context of the specifier.", GH_ParamAccess.item);
        pManager.AddTextParameter("Group", "Gr?", "Optional group of the specifier. No group means true.",
            GH_ParamAccess.item);
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
        if (specifierGoo.Value.Context == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A specifier needs a context.");
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
        if (portGoo.Value.Plane.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs a plane.");
            isValidInput = false;
        }

        if (portGoo.Value.Specifiers.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs at least one specifier.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, portGoo.Duplicate());
        DA.SetData(1, portGoo.Value.Plane.convert());
        DA.SetDataList(2, portGoo.Value.Specifiers.Select(s => new SpecifierGoo(s.DeepClone()
        )));
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
        pManager.AddTextParameter("Value", "Va?", "Optional value of the quality. No value means true.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Unit", "Un?", " Optional unit of the quality.", GH_ParamAccess.item);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
            "Constructed or modified quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va?", "Optional value of the quality. No value means true.",
            GH_ParamAccess.item);
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
        if (qualityGoo.Value.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A quality needs a name.");
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
        pManager.AddTextParameter("Unit", "Ut",
            "Unit of the type. By default the document unit is used. Otherwise meters will be used.",
            GH_ParamAccess.item);
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
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                typeGoo.Value.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                typeGoo.Value.Unit = "m";
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
        if (typeGoo.Value.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs a name.");
            isValidInput = false;
        }

        // currently impossible
        if (typeGoo.Value.Unit == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs a unit.");
            isValidInput = false;
        }

        if (!Utility.IsValidUnit(typeGoo.Value.Unit))
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The unit is not valid.");
            isValidInput = false;
        }

        if (typeGoo.Value.Representations.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs at least one representation.");
            isValidInput = false;
        }

        if (typeGoo.Value.Ports.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A type needs at least one port.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, typeGoo.Duplicate());
        DA.SetData(1, typeGoo.Value.Name);
        DA.SetData(2, typeGoo.Value.Description);
        DA.SetData(3, typeGoo.Value.Icon);
        DA.SetData(4, typeGoo.Value.Variant);
        DA.SetData(5, typeGoo.Value.Unit);
        DA.SetDataList(6, typeGoo.Value.Representations.Select(r => new RepresentationGoo(r.DeepClone())));
        DA.SetDataList(7, typeGoo.Value.Ports.Select(p => new PortGoo(p.DeepClone())));
        DA.SetDataList(8, typeGoo.Value.Qualities.Select(q => new QualityGoo(q.DeepClone())));
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
            "Id of the piece.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Type Name", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Type Variant", "TyVn?", "Optional variant of the type of the piece.",
            GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddPlaneParameter("Root Plane", "RtPn?",
            "Root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never attracted.",
            GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddParameter(new ScreenPointParam(), "Diagram Screen Point", "DgSP",
            "Screen point of the piece in the diagram.",
            GH_ParamAccess.item);
        pManager[5].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "Pc",
            "Constructed or modified piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id", "Id of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Name", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Variant", "TyVn?", "Optional variant of the type of the piece.",
            GH_ParamAccess.item);
        pManager.AddPlaneParameter("Root Plane", "RtPl?",
            "Root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never attracted.",
            GH_ParamAccess.item);
        pManager.AddParameter(new ScreenPointParam(), "Diagram Screen Point", "DgSP",
            "Screen point of the piece in the diagram.",
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

        DA.GetData(0, ref pieceGoo);
        if (DA.GetData(1, ref id))
            pieceGoo.Value.Id = id;
        if (DA.GetData(2, ref typeName))
            pieceGoo.Value.Type.Name = typeName;
        if (DA.GetData(3, ref typeVariant))
            pieceGoo.Value.Type.Variant = typeVariant;
        if (DA.GetData(4, ref rootPlane))
            pieceGoo.Value.Root = new RootPiece { Plane = rootPlane.convert() };
        if (DA.GetData(5, ref screenPointGoo))
            pieceGoo.Value.Diagram.Point = screenPointGoo.Value;

        var isValidInput = true;
        if (pieceGoo.Value.Id == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A piece needs an id.");
            isValidInput = false;
        }

        if (pieceGoo.Value.Type.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A piece needs a type name.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, pieceGoo.Duplicate());
        DA.SetData(1, pieceGoo.Value.Id);
        DA.SetData(2, pieceGoo.Value.Type.Name);
        DA.SetData(3, pieceGoo.Value.Type.Variant);
        DA.SetData(4, pieceGoo.Value.Root?.Plane.convert());
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
            sideGoo.Value.Piece.Type.Port.Specifiers = pieceTypePortSpecifiers.Select(s => s.Value).ToList();

        var isValidInput = true;
        if (sideGoo.Value.Piece.Id == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A side needs a piece id.");
            isValidInput = false;
        }

        if (sideGoo.Value.Piece.Type.Port.Specifiers.Count == 0)
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

        var isValidInput = true;
        if (attractionGoo.Value.Attracting.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "An attraction needs an attracting side.");
            isValidInput = false;
        }

        if (attractionGoo.Value.Attracted.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "An attraction needs an attracted side.");
            isValidInput = false;
        }

        if (attractionGoo.Value.Attracting.Piece.Id == attractionGoo.Value.Attracted.Piece.Id)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "An attraction cannot attract itself.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, attractionGoo.Duplicate());
        DA.SetData(1, new SideGoo(attractionGoo.Value.Attracting.DeepClone()));
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
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                formationGoo.Value.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                formationGoo.Value.Unit = "m";
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
        if (formationGoo.Value.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs a name.");
            isValidInput = false;
        }

        // currently impossible
        if (formationGoo.Value.Unit == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs a unit.");
            isValidInput = false;
        }

        if (!Utility.IsValidUnit(formationGoo.Value.Unit))
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The unit is not valid.");
            isValidInput = false;
        }

        if (formationGoo.Value.Pieces.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A formation needs at least one piece.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, formationGoo.Duplicate());
        DA.SetData(1, formationGoo.Value.Name);
        DA.SetData(2, formationGoo.Value.Description);
        DA.SetData(3, formationGoo.Value.Icon);
        DA.SetData(4, formationGoo.Value.Variant);
        DA.SetData(5, formationGoo.Value.Unit);
        DA.SetDataList(6, formationGoo.Value.Pieces.Select(p => new PieceGoo(p.DeepClone())));
        DA.SetDataList(7, formationGoo.Value.Attractions.Select(a => new AttractionGoo(a.DeepClone())));
        DA.SetDataList(8, formationGoo.Value.Qualities.Select(q => new QualityGoo(q.DeepClone())));
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
            Name = formationName
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