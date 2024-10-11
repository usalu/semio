using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net;
using System.Net.NetworkInformation;
using System.Net.Sockets;
using System.Reflection;
using System.Security.Cryptography;
using System.Text;
using GH_IO.Serialization;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino;
using Rhino.Geometry;

namespace Semio.Grasshopper;

// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();
// TODO: IsInvalid is used to check null state which is not clean.
// Think of a better way to handle this.
// The invalid check happen twice and code is duplicated.

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
//        Mime = "";
//        Lod = "";
//        Tags = new List<string>();
//    }

//    public string Url { get; set; }
//    public string Mime { get; set; }
//    public string Lod { get; set; }
//    public List<string> Tags { get; set; }

//    public Representation DeepClone()
//    {
//        return new Representation
//        {
//            Url = Url,
//            Mime = Mime,
//            Lod = Lod,
//            Tags = new List<string>(Tags)
//        };
//    }

//    public override string ToString()
//    {
//        return $"Representation(Url:{Url})";
//    }

//    public bool IsInvalid()
//    {
//        return Url == "";
//    }
//}

//public class Locator : IDeepCloneable<Locator>, IEntity
//{
//    public Locator()
//    {
//        Group = "";
//        Subgroup = "";
//    }

//    public string Group { get; set; }
//    public string Subgroup { get; set; }

//    public Locator DeepClone()
//    {
//        return new Locator
//        {
//            Group = Group,
//            Subgroup = Subgroup
//        };
//    }

//    public override string ToString()
//    {
//        return $"Locator(Group:{Group}" + (Subgroup != "" ? $",Subgroup:{Subgroup})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return Group == "";
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
//        return $"Point(X:{X},Y:{Y})";
//    }

//    public bool IsInvalid()
//    {
//        return false;
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
//        return $"Point(X:{X},Y:{Y},Z:{Z})";
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
//        return $"Vector(X:{X},Y:{Y},Z:{Z})";
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
//        return $"Plane(Origin:{Origin},XAxis:{XAxis},YAxis: {YAxis})";
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
//        Id = "";
//        Point = new Point();
//        Direction = new Vector();
//        Locators = new List<Locator>();
//    }

//    public string Id { get; set; }
//    public Point Point { get; set; }
//    public Vector Direction { get; set; }
//    public List<Locator> Locators { get; set; }

//    public Port DeepClone()
//    {
//        return new Port
//        {
//            Id = Id,
//            Point = Point.DeepClone(),
//            Direction = Direction.DeepClone(),
//            Locators = new List<Locator>(Locators.Select(s => s.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return "Port(" + (Id != "" ? $"Id:{Id})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return Id == "" || Point.IsInvalid() || Direction.IsInvalid() || Locators.Any(s => s.IsInvalid());
//    }
//}

//public class PortId : IDeepCloneable<PortId>, IEntity
//{
//    public PortId()
//    {
//        Id = "";
//    }

//    public string Id { get; set; }

//    public PortId DeepClone()
//    {
//        return new PortId
//        {
//            Id = Id
//        };
//    }

//    public override string ToString()
//    {
//        return "Port(" + (Id != "" ? $"Id:{Id})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return false;
//    }
//}


//public class Quality : IDeepCloneable<Quality>, IEntity
//{
//    public Quality()
//    {
//        Name = "";
//        Value = "";
//        Unit = "";
//        Definition = "";
//    }

//    public string Name { get; set; }
//    public string Value { get; set; }
//    public string Unit { get; set; }
//    public string Definition { get; set; }

//    public Quality DeepClone()
//    {
//        return new Quality
//        {
//            Name = Name,
//            Value = Value,
//            Unit = Unit,
//            Definition = Definition
//        };
//    }

//    public override string ToString()
//    {
//        return $"Quality(Name:{Name})";
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
//        return $"Type(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
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
//        return $"Type(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return Name == "";
//    }
//}

//public class PieceRoot : IDeepCloneable<PieceRoot>, IEntity
//{
//    public PieceRoot()
//    {
//        Plane = new Plane();
//    }

//    public Plane Plane { get; set; }

//    public PieceRoot DeepClone()
//    {
//        return new PieceRoot
//        {
//            Plane = Plane.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Root({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Plane.IsInvalid();
//    }
//}

//public class PieceDiagram : IDeepCloneable<PieceDiagram>, IEntity
//{
//    public PieceDiagram()
//    {
//        Point = new ScreenPoint();
//    }

//    public ScreenPoint Point { get; set; }

//    public PieceDiagram DeepClone()
//    {
//        return new PieceDiagram
//        {
//            Point = Point.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Diagram({Point})";
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
//        Diagram = new PieceDiagram();
//    }

//    public string Id { get; set; }
//    public TypeId Type { get; set; }
//    public PieceRoot? Root { get; set; }
//    public PieceDiagram Diagram { get; set; }

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
//        return $"Piece(Id:{Id})";
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
//        return $"Piece(Id:{Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "";
//    }
//}

//public class SidePieceType : IDeepCloneable<SidePieceType>, IEntity
//{
//    public SidePieceType()
//    {
//        Port = new PortId();
//    }

//    public PortId Port { get; set; }

//    public SidePieceType DeepClone()
//    {
//        return new SidePieceType
//        {
//            Port = Port.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Type({Port})";
//    }

//    public bool IsInvalid()
//    {
//        return Port.IsInvalid();
//    }
//}

//public class SidePiece : IDeepCloneable<SidePiece>, IEntity
//{
//    public SidePiece()
//    {
//        Id = "";
//        Type = new SidePieceType();
//    }

//    public string Id { get; set; }
//    public SidePieceType Type { get; set; }

//    public SidePiece DeepClone()
//    {
//        return new SidePiece
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Piece(Id:{Id}" + (Type.Port.Id != "" ? $",{Type})" : ")");
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
//        Piece = new SidePiece();
//    }

//    public SidePiece Piece { get; set; }

//    public Side DeepClone()
//    {
//        return new Side
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Side({Piece})";
//    }

//    public bool IsInvalid()
//    {
//        return Piece.IsInvalid();
//    }
//}


//public class Connection : IDeepCloneable<Connection>, IEntity
//{
//    public Connection()
//    {
//        Connected = new Side();
//        Connecting = new Side();
//        Offset = 0;
//        Rotation = 0;
//    }

//    public Side Connected { get; set; }
//    public Side Connecting { get; set; }
//    public float Offset { get; set; }
//    public float Rotation { get; set; }

//    public Connection DeepClone()
//    {
//        return new Connection
//        {
//            Connected = Connected.DeepClone(),
//            Connecting = Connecting.DeepClone(),
//            Offset = Offset,
//            Rotation = Rotation
//        };
//    }

//    public override string ToString()
//    {
//        return $"Connection(Connected({Connected}),Connecting({Connecting}),Offset:{Offset},Rotation:{Rotation})";
//    }

//    public bool IsInvalid()
//    {
//        return Connecting.IsInvalid() || Connected.IsInvalid() || Connecting.Piece.Id == Connected.Piece.Id;
//    }
//}

//public class Design : IDeepCloneable<Design>, IEntity
//{
//    public Design()
//    {
//        Name = "";
//        Description = "";
//        Icon = "";
//        Variant = "";
//        Unit = "";
//        Pieces = new List<Piece>();
//        Connections = new List<Connection>();
//        Qualities = new List<Quality>();
//    }

//    public string Name { get; set; }
//    public string Description { get; set; }
//    public string Icon { get; set; }
//    public string Variant { get; set; }
//    public string Unit { get; set; }
//    public List<Piece> Pieces { get; set; }
//    public List<Connection> Connections { get; set; }
//    public List<Quality> Qualities { get; set; }

//    public Design DeepClone()
//    {
//        return new Design
//        {
//            Name = Name,
//            Description = Description,
//            Icon = Icon,
//            Variant = Variant,
//            Unit = Unit,
//            Pieces = new List<Piece>(Pieces.Select(p => p.DeepClone())),
//            Connections = new List<Connection>(Connections.Select(a => a.DeepClone())),
//            Qualities = new List<Quality>(Qualities.Select(q => q.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Design(Name:{Name}" + (Variant != "" ? $",Variant: {Variant})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return Name == "" || Unit == "" || Pieces.Any(p => p.IsInvalid()) || Connections.Any(a => a.IsInvalid()) ||
//               Qualities.Any(q => q.IsInvalid());
//    }
//}

//public class DesignId : IDeepCloneable<DesignId>, IEntity
//{
//    public DesignId()
//    {
//        Name = "";
//        Variant = "";
//    }

//    public string Name { get; set; }
//    public string Variant { get; set; }

//    public DesignId DeepClone()
//    {
//        return new DesignId
//        {
//            Name = Name,
//            Variant = Variant
//        };
//    }

//    public override string ToString()
//    {
//        return $"Design(Name:{Name}" + (Variant != "" ? $",Variant:{Variant})" : ")");
//    }

//    public bool IsInvalid()
//    {
//        return Name == "";
//    }
//}

//public class ObjectPieceType : IDeepCloneable<ObjectPieceType>, IEntity
//{
//    public ObjectPieceType()
//    {
//        Representations = new List<Representation>();
//    }

//    public List<Representation> Representations { get; set; }

//    public ObjectPieceType DeepClone()
//    {
//        return new ObjectPieceType
//        {
//            Representations = new List<Representation>(Representations.Select(f => f.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Type({GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Representations.Any(r => r.IsInvalid());
//    }
//}

//public class ObjectPiece : IDeepCloneable<ObjectPiece>, IEntity
//{
//    public ObjectPiece()
//    {
//        Id = "";
//        Type = new ObjectPieceType();
//    }

//    public string Id { get; set; }
//    public ObjectPieceType Type { get; set; }

//    public ObjectPiece DeepClone()
//    {
//        return new ObjectPiece
//        {
//            Id = Id,
//            Type = Type.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Piece(Id:{Id})";
//    }

//    public bool IsInvalid()
//    {
//        return Id == "" || Type.IsInvalid();
//    }
//}


//public class ObjectParent : IDeepCloneable<ObjectParent>, IEntity
//{
//    public ObjectParent()
//    {
//        Piece = new PieceId();
//    }

//    public PieceId Piece { get; set; }

//    public ObjectParent DeepClone()
//    {
//        return new ObjectParent
//        {
//            Piece = Piece.DeepClone()
//        };
//    }

//    public override string ToString()
//    {
//        return $"Parent({Piece})";
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
//        Piece = new ObjectPiece();
//        Plane = new Plane();
//        Parent = null;
//    }

//    public ObjectPiece Piece { get; set; }
//    public Plane Plane { get; set; }
//    public ObjectParent? Parent { get; set; }

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
//        return $"Object({Piece})";
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
//        Design = new DesignId();
//        Objects = new List<Object>();
//    }

//    public DesignId Design { get; set; }
//    public List<Object> Objects { get; set; }

//    public Scene DeepClone()
//    {
//        return new Scene
//        {
//            Design = Design.DeepClone(),
//            Objects = new List<Object>(Objects.Select(o => o.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Scene({Design})";
//    }

//    public bool IsInvalid()
//    {
//        return Design.IsInvalid() || Objects.Any(o => o.IsInvalid());
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
//        Homepage = "";
//        Types = new List<Type>();
//        Designs = new List<Design>();
//    }

//    public string Name { get; set; }
//    public string Description { get; set; }
//    public string Icon { get; set; }
//    public string Url { get; set; }
//    public string Homepage { get; set; }
//    public List<Type> Types { get; set; }
//    public List<Design> Designs { get; set; }

//    public Kit DeepClone()
//    {
//        return new Kit
//        {
//            Name = Name,
//            Description = Description,
//            Icon = Icon,
//            Url = Url,
//            Homepage = Homepage,
//            Types = new List<Type>(Types.Select(t => t.DeepClone())),
//            Designs = new List<Design>(Designs.Select(f => f.DeepClone()))
//        };
//    }

//    public override string ToString()
//    {
//        return $"Kit(Name:{Name}, {GetHashCode()})";
//    }

//    public bool IsInvalid()
//    {
//        return Name == "" || Types.Any(t => t.IsInvalid()) || Designs.Any(f => f.IsInvalid());
//    }
//}

//public class KitMetadata : IDeepCloneable<KitMetadata>, IEntity
//{
//    public string? Name { get; set; }
//    public string? Description { get; set; }
//    public string? Icon { get; set; }
//    public string? Url { get; set; }
//    public string? Homepage { get; set; }

//    public KitMetadata DeepClone()
//    {
//        var kitMetadata = new KitMetadata();
//        if (Name != null) kitMetadata.Name = Name;
//        if (Description != null) kitMetadata.Description = Description;
//        if (Icon != null) kitMetadata.Icon = Icon;
//        if (Url != null) kitMetadata.Url = Url;
//        if (Homepage != null) kitMetadata.Homepage = Homepage;
//        return kitMetadata;
//    }

//    public override string ToString()
//    {
//        return $"Kit(Name:{Name})";
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

    public static Rhino.Geometry.Plane GetPlaneFromYAxis(Vector3d yAxis, float theta, Point3d origin)
    {
        var thetaRad = RhinoMath.ToRadians(theta);
        var orientation = Transform.Rotation(Vector3d.YAxis, yAxis, Point3d.Origin);
        var rotation = Transform.Rotation(thetaRad, yAxis, Point3d.Origin);
        var xAxis = Vector3d.XAxis;
        xAxis.Transform(rotation * orientation);
        return new Rhino.Geometry.Plane(origin, xAxis, yAxis);
    }
}

#endregion

#region Converters

public static class RhinoConverter
{
    public static Point3d convert(this Point point)
    {
        return new Point3d(point.X, point.Y, point.Z);
    }

    public static Point convert(this Point3d point)
    {
        return new Point
        {
            X = (float)point.X,
            Y = (float)point.Y,
            Z = (float)point.Z
        };
    }

    public static Vector3d convert(this Vector vector)
    {
        return new Vector3d(vector.X, vector.Y, vector.Z);
    }

    public static Vector convert(this Vector3d vector)
    {
        return new Vector
        {
            X = (float)vector.X,
            Y = (float)vector.Y,
            Z = (float)vector.Z
        };
    }

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

public class LocatorGoo : GH_Goo<Locator>
{
    public LocatorGoo()
    {
        Value = new Locator();
    }

    public LocatorGoo(Locator locator)
    {
        Value = locator;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Locator";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new LocatorGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Locator", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Locator").Deserialize<Locator>();
        return base.Read(reader);
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Group);
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
            Value = new Locator
            {
                Group = str
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
            object ptr = new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.convert(), 0, Value.Point.convert()));
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
            Value.Point = plane.Origin.convert();
            Value.Direction = plane.YAxis.convert();

            return true;
        }

        string str = null;
        if (GH_Convert.ToString(source, out str, GH_Conversion.Both))
        {
            Value = str.Deserialize<Port>();
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

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Piece.Id);
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
            Value = new Side
            {
                Piece = new SidePiece
                {
                    Id = str
                }
            };
            return true;
        }

        if (source is Piece piece)
        {
            Value = new Side
            {
                Piece = new SidePiece
                {
                    Id = piece.Id
                }
            };
            return true;
        }

        return false;
    }
}

public class ConnectionGoo : GH_Goo<Connection>
{
    public ConnectionGoo()
    {
        Value = new Connection();
    }

    public ConnectionGoo(Connection connection)
    {
        Value = connection;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Connection";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new ConnectionGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Connection", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Connection").Deserialize<Connection>();
        return base.Read(reader);
    }
}

// TODO: Implement cast with type
public class DesignGoo : GH_Goo<Design>
{
    public DesignGoo()
    {
        Value = new Design();
    }

    public DesignGoo(Design design)
    {
        Value = design;
    }

    public override bool IsValid { get; }
    public override string TypeName => "Design";
    public override string TypeDescription { get; }

    public override IGH_Goo Duplicate()
    {
        return new DesignGoo(Value.DeepClone());
    }

    public override string ToString()
    {
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString("Design", Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString("Design").Deserialize<Design>();
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

public class LocatorParam : SemioPersistentParam<LocatorGoo>
{
    public LocatorParam() : base("Locator", "Lc", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("DBE104DA-63FA-4C68-8D41-834DD962F1D7");

    protected override Bitmap Icon => Resources.locator_24x24;

    protected override GH_GetterResult Prompt_Singular(ref LocatorGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<LocatorGoo> values)
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

public class ConnectionParam : SemioPersistentParam<ConnectionGoo>
{
    public ConnectionParam() : base("Connection", "Co", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");

    protected override Bitmap Icon => Resources.connection_24x24;

    protected override GH_GetterResult Prompt_Singular(ref ConnectionGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<ConnectionGoo> values)
    {
        throw new NotImplementedException();
    }
}

public class DesignParam : SemioPersistentParam<DesignGoo>
{
    public DesignParam() : base("Design", "Dn", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");

    protected override Bitmap Icon => Resources.design_24x24;

    protected override GH_GetterResult Prompt_Singular(ref DesignGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<DesignGoo> values)
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

public class KitParam : SemioPersistentParam<KitGoo>
{
    public KitParam() : base("Kit", "Kt", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");

    protected override Bitmap Icon => Resources.kit_24x24;

    protected override GH_GetterResult Prompt_Singular(ref KitGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<KitGoo> values)
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
        pManager.AddTextParameter("Mime", "Mm", "Mime type of the representation.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Level of Detail", "Ld?",
            "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \nThere can be only one default representation per type.",
            GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list,
            new List<string>());
        pManager[4].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp",
            "Constructed or modified representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "Url of the representation. Either a relative file path or link.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Mime", "Mm", "Mime type of the representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("LoD", "Ld?",
            "Optional LoD(Level of Detail / Development / Design / ...) of the representation. No LoD means default. \\nThere can be only one default representation per type.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var representationGoo = new RepresentationGoo();
        var url = "";
        var mime = "";
        var lod = "";
        var tags = new List<string>();

        if (DA.GetData(0, ref representationGoo))
            representationGoo = representationGoo.Duplicate() as RepresentationGoo;
        if (DA.GetData(1, ref url))
            representationGoo.Value.Url = url;
        if (!DA.GetData(2, ref mime))
        {
            if (representationGoo.Value.Mime == "")
                representationGoo.Value.Mime = MimeParser.ParseFromUrl(representationGoo.Value.Url);
        }
        else
            representationGoo.Value.Mime = mime;
        if (DA.GetData(3, ref lod))
            representationGoo.Value.Lod = lod;
        if (DA.GetDataList(4, tags))
            representationGoo.Value.Tags = tags;

        var isValidInput = true;
        if (representationGoo.Value.Url == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A representation needs an url.");
            isValidInput = false;
        }
        if (representationGoo.Value.Mime == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The mime type couldn't be guessed by the url. Please provide it.");
            isValidInput = false;
        }
        if (!isValidInput) return;

        DA.SetData(0, representationGoo.Duplicate());
        DA.SetData(1, representationGoo.Value.Url);
        DA.SetData(2, representationGoo.Value.Mime);
        DA.SetData(3, representationGoo.Value.Lod);
        DA.SetDataList(4, representationGoo.Value.Tags);
    }
}

public class LocatorComponent : SemioComponent
{
    public LocatorComponent()
        : base("Model Locator", "~Loc",
            "Construct, deconstruct or modify a locator.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("2552DB71-8459-4DB5-AD66-723573E771A2");

    protected override Bitmap Icon => Resources.locator_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new LocatorParam(), "Locator", "Sp?",
            "Optional locator to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Group", "Gr", "Group of the locator.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Subgroup", "SG?", "Optional subgroup of the locator.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new LocatorParam(), "Locator", "Lc",
            "Constructed or modified locator.", GH_ParamAccess.item);
        pManager.AddTextParameter("Group", "Gr", "Group of the locator.", GH_ParamAccess.item);
        pManager.AddTextParameter("Subgroup", "SG?", "Optional subgroup of the locator.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var locatorGoo = new LocatorGoo();
        var group = "";
        var subgroup = "";

        if (DA.GetData(0, ref locatorGoo))
            locatorGoo = locatorGoo.Duplicate() as LocatorGoo;
        if (DA.GetData(1, ref group))
            locatorGoo.Value.Group = group;
        if (DA.GetData(2, ref subgroup))
            locatorGoo.Value.Subgroup = subgroup;

        var isValidInput = true;
        if (locatorGoo.Value.Group == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A locator needs a group.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, locatorGoo.Duplicate());
        DA.SetData(1, locatorGoo.Value.Group);
        DA.SetData(2, locatorGoo.Value.Subgroup);
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
        pManager.AddTextParameter("Id", "Id?", "Optional id of the port.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddPointParameter("Point", "Pt", "Point of the port.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddVectorParameter("Direction", "Di", "Direction of the port.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new LocatorParam(), "Locators", "Lc*",
            "Optional locators of the port. Locators help to understand the location of the port. Every port should have a set of unique locators.",
            GH_ParamAccess.list);
        pManager[4].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PortParam(), "Port", "Po",
            "Constructed or modified port.", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id", "Id of the port.", GH_ParamAccess.item);
        pManager.AddPointParameter("Point", "Pt", "Point of the port.", GH_ParamAccess.item);
        pManager.AddVectorParameter("Direction", "Di", "Direction of the port.", GH_ParamAccess.item);
        pManager.AddParameter(new LocatorParam(), "Locators", "Lc*",
            "Optional locators of the port. Locators help to understand the location of the port. Every port should have a set of unique locators.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var portGoo = new PortGoo();
        var id = "";
        var pointGeo = new Point3d();
        var directionGeo = new Vector3d();
        var locatorGoos = new List<LocatorGoo>();

        if (DA.GetData(0, ref portGoo))
            portGoo = portGoo.Duplicate() as PortGoo;
        if (DA.GetData(1, ref id))
            portGoo.Value.Id = id;
        if (DA.GetData(2, ref pointGeo))
            portGoo.Value.Point = pointGeo.convert();
        if (DA.GetData(3, ref directionGeo))
            portGoo.Value.Direction = directionGeo.convert();
        if (DA.GetDataList(4, locatorGoos))
            portGoo.Value.Locators = locatorGoos.Select(s => s.Value).ToList();

        var isValidInput = true;
        if (portGoo.Value.Point.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs a point.");
            isValidInput = false;
        }

        if (portGoo.Value.Direction.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A port needs a direction.");
            isValidInput = false;
        }

        if (portGoo.Value.Direction.IsZero())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The direction needs to point somewhere.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, portGoo.Duplicate());
        DA.SetData(1, portGoo.Value.Id);
        DA.SetData(2, portGoo.Value.Point.convert());
        DA.SetData(3, portGoo.Value.Direction.convert());
        DA.SetDataList(4, portGoo.Value.Locators.Select(s => new LocatorGoo(s.DeepClone()
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
        pManager.AddTextParameter("Definition", "Df?", "Optional definition of the quality.", GH_ParamAccess.item);
        pManager[4].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
            "Constructed or modified quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Value", "Va?", "Optional value of the quality. No value means true.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Un?", "Optional unit of the quality.", GH_ParamAccess.item);
        pManager.AddTextParameter("Definition", "Df?", "Optional definition of the quality.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var qualityGoo = new QualityGoo();
        var name = "";
        var value = "";
        var unit = "";
        var definition = "";

        if (DA.GetData(0, ref qualityGoo))
            qualityGoo = qualityGoo.Duplicate() as QualityGoo;
        if (DA.GetData(1, ref name))
            qualityGoo.Value.Name = name;
        if (DA.GetData(2, ref value))
            qualityGoo.Value.Value = value;
        if (DA.GetData(3, ref unit))
            qualityGoo.Value.Unit = unit;
        if (DA.GetData(4, ref definition))
            qualityGoo.Value.Definition = definition;

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
        DA.SetData(4, qualityGoo.Value.Definition);
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
        pManager.AddTextParameter("Variant", "Vn?",
            "Optional variant of the type. No variant means the default variant. There can be only one default variant.",
            GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Unit", "Ut",
            "Unit of the type. By default the document unit is used. Otherwise meters will be used.",
            GH_ParamAccess.item);
        pManager[5].Optional = true;
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+", "Representations of the type.",
            GH_ParamAccess.list);
        pManager[6].Optional = true;
        pManager.AddParameter(new PortParam(), "Ports", "Po+", "Ports of the type.", GH_ParamAccess.list);
        pManager[7].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the type. A quality is meta-data for decision making.",
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
        pManager.AddTextParameter("Variant", "Vn?",
            "Optional variant of the type. No variant means the default variant. There can be only one default variant.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut",
            "Unit of the type. By default the document unit is used. Otherwise meters will be used.",
            GH_ParamAccess.item);
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+", "Representations of the type",
            GH_ParamAccess.list);
        pManager.AddParameter(new PortParam(), "Ports", "Po+", "Ports of the type.", GH_ParamAccess.list);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the type. A quality is meta-data for decision making.",
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

        if (DA.GetData(0, ref typeGoo))
            typeGoo = typeGoo.Duplicate() as TypeGoo;
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

        if (DA.GetData(0, ref screenPointGoo))
            screenPointGoo = screenPointGoo.Duplicate() as ScreenPointGoo;
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
        pManager.AddTextParameter("Type Variant", "TyVn?",
            "Optional variant of the type of the piece. No variant means the default variant.",
            GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddPlaneParameter("Root Plane", "RtPn?",
            "Root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never connected.",
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
        pManager.AddTextParameter("Type Variant", "TyVn?",
            "Optional variant of the type of the piece. No variant means the default variant.",
            GH_ParamAccess.item);
        pManager.AddPlaneParameter("Root Plane", "RtPn?",
            "Root plane of the piece. This only applies to root pieces. \nA piece is a root piece when it is never connected.",
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

        if (DA.GetData(0, ref pieceGoo))
            pieceGoo = pieceGoo.Duplicate() as PieceGoo;
        if (DA.GetData(1, ref id))
            pieceGoo.Value.Id = id;
        if (DA.GetData(2, ref typeName))
            pieceGoo.Value.Type.Name = typeName;
        if (DA.GetData(3, ref typeVariant))
            pieceGoo.Value.Type.Variant = typeVariant;
        if (DA.GetData(4, ref rootPlane))
            pieceGoo.Value.Root = new PieceRoot { Plane = rootPlane.convert() };
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
        pManager.AddTextParameter("Piece Type Port Id", "PcTyPoId?",
            "Optional id of the port of type of the piece of the side. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SideParam(), "Side", "Sd",
            "Constructed or modified side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Piece Id", "PcId", "Id of the piece of the side.", GH_ParamAccess.item);
        pManager.AddParameter(new LocatorParam(), "Piece Type Port Id", "PcTyPoId?",
            "Optional id of the port of type of the piece of the side. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var sideGoo = new SideGoo();
        var pieceId = "";
        var pieceTypePortId = "";

        if (DA.GetData(0, ref sideGoo))
            sideGoo = sideGoo.Duplicate() as SideGoo;
        if (DA.GetData(1, ref pieceId))
            sideGoo.Value.Piece.Id = pieceId;
        if (DA.GetData(2, ref pieceTypePortId))
            sideGoo.Value.Piece.Type.Port.Id = pieceTypePortId;

        var isValidInput = true;
        if (sideGoo.Value.Piece.Id == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A side needs a piece id.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, sideGoo.Duplicate());
        DA.SetData(1, sideGoo.Value.Piece.Id);
        DA.SetData(2, sideGoo.Value.Piece.Type.Port.Id);
    }
}

public class ConnectionComponent : SemioComponent
{
    public ConnectionComponent()
        : base("Model Connection", "~Con",
            "Construct, deconstruct or modify an connection.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");

    protected override Bitmap Icon => Resources.connection_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "Co?",
            "Optional connection to deconstruct or modify.", GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddParameter(new SideParam(), "Connected Side", "CdSd", "Connected side of the connection.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddParameter(new SideParam(), "Connecting Side", "CgSd", "Connecting side of the connection.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddNumberParameter("Offset", "Of?", "Optional offset (in port direction) of the connection.",
            GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddNumberParameter("Rotation", "Rt?",
            "Optional rotation (degree around the port direction) of the connection.",
            GH_ParamAccess.item);
        pManager[4].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new ConnectionParam(), "Connection", "Co",
            "Constructed or modified connection.", GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connected Side", "CdSd", "Connected side of the connection.",
            GH_ParamAccess.item);
        pManager.AddParameter(new SideParam(), "Connecting Side", "CgSd", "Connecting side of the connection.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Offset", "Of?", "Optional offset (in port direction) of the connection.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?",
            "Optional rotation (degree around the port direction) of the connection.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var connectionGoo = new ConnectionGoo();
        var connectedSideGoo = new SideGoo();
        var connectingSideGoo = new SideGoo();
        var offset = 0.0;
        var rotation = 0.0;

        if (DA.GetData(0, ref connectionGoo))
            connectionGoo = connectionGoo.Duplicate() as ConnectionGoo;
        if (DA.GetData(1, ref connectedSideGoo)) connectionGoo.Value.Connected = connectedSideGoo.Value;
        if (DA.GetData(2, ref connectingSideGoo)) connectionGoo.Value.Connecting = connectingSideGoo.Value;
        if (DA.GetData(3, ref offset)) connectionGoo.Value.Offset = (float)offset;
        if (DA.GetData(4, ref rotation)) connectionGoo.Value.Rotation = (float)rotation;

        var isValidInput = true;
        if (connectionGoo.Value.Connected.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "An connection needs an connected side.");
            isValidInput = false;
        }

        if (connectionGoo.Value.Connecting.IsInvalid())
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "An connection needs an connecting side.");
            isValidInput = false;
        }

        if (connectionGoo.Value.Connecting.Piece.Id != "" &&
            connectionGoo.Value.Connected.Piece.Id == connectionGoo.Value.Connecting.Piece.Id)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "An connection cannot attract itself.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, connectionGoo.Duplicate());
        DA.SetData(1, new SideGoo(connectionGoo.Value.Connected?.DeepClone()));
        DA.SetData(2, new SideGoo(connectionGoo.Value.Connecting.DeepClone()));
        DA.SetData(3, connectionGoo.Value.Offset);
        DA.SetData(4, connectionGoo.Value.Rotation);
    }
}

public class DesignComponent : SemioComponent
{
    public DesignComponent()
        : base("Model Design", "~Dsn",
            "Construct, deconstruct or modify a design.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");

    protected override Bitmap Icon => Resources.design_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn?",
            "Optional design to deconstruct or modify. A design is a collection of pieces that are connected.",
            GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "Na", "Name of the design.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the design.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the design.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddTextParameter("Variant", "Vn?",
            "Optional variant of the design. No variant means the default variant. There can be only one default variant.",
            GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Unit", "Ut", "Unit of the design.", GH_ParamAccess.item);
        pManager[5].Optional = true;
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc+", "Pieces of the design.", GH_ParamAccess.list);
        pManager[6].Optional = true;
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "Optional connections of the design.",
            GH_ParamAccess.list);
        pManager[7].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the design. A quality is meta-data for decision making.",
            GH_ParamAccess.list);
        pManager[8].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Constructed or modified design. A design is a collection of pieces that are connected.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the design.", GH_ParamAccess.item);
        pManager.AddTextParameter("Variant", "Vn?",
            "Optional variant of the design. No variant means the default variant. There can be only one default variant.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Unit", "Ut", "Unit of the design.", GH_ParamAccess.item);
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc+", "Pieces of the design.", GH_ParamAccess.list);
        pManager.AddParameter(new ConnectionParam(), "Connections", "Co*", "Optional connections of the design.",
            GH_ParamAccess.list);
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the design. A quality is meta-data for decision making.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designGoo = new DesignGoo();
        var name = "";
        var description = "";
        var icon = "";
        var variant = "";
        var unit = "";
        var pieceGoos = new List<PieceGoo>();
        var connectionGoos = new List<ConnectionGoo>();
        var qualityGoos = new List<QualityGoo>();

        if (DA.GetData(0, ref designGoo))
            designGoo = designGoo.Duplicate() as DesignGoo;
        if (DA.GetData(1, ref name))
            designGoo.Value.Name = name;
        if (DA.GetData(2, ref description))
            designGoo.Value.Description = description;
        if (DA.GetData(3, ref icon))
            designGoo.Value.Icon = icon;
        if (DA.GetData(4, ref variant))
            designGoo.Value.Variant = variant;
        if (!DA.GetData(5, ref unit))
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                designGoo.Value.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                designGoo.Value.Unit = "m";
            }
        else
            designGoo.Value.Unit = unit;

        if (DA.GetDataList(6, pieceGoos))
            designGoo.Value.Pieces = pieceGoos.Select(p => p.Value).ToList();
        if (DA.GetDataList(7, connectionGoos))
            designGoo.Value.Connections = connectionGoos.Select(a => a.Value).ToList();
        if (DA.GetDataList(8, qualityGoos))
            designGoo.Value.Qualities = qualityGoos.Select(q => q.Value).ToList();

        var isValidInput = true;
        if (designGoo.Value.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A design needs a name.");
            isValidInput = false;
        }

        // currently impossible
        if (designGoo.Value.Unit == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A design needs a unit.");
            isValidInput = false;
        }

        if (!Utility.IsValidUnit(designGoo.Value.Unit))
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "The unit is not valid.");
            isValidInput = false;
        }

        if (designGoo.Value.Pieces.Count == 0)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A design needs at least one piece.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, designGoo.Duplicate());
        DA.SetData(1, designGoo.Value.Name);
        DA.SetData(2, designGoo.Value.Description);
        DA.SetData(3, designGoo.Value.Icon);
        DA.SetData(4, designGoo.Value.Variant);
        DA.SetData(5, designGoo.Value.Unit);
        DA.SetDataList(6, designGoo.Value.Pieces.Select(p => new PieceGoo(p.DeepClone())));
        DA.SetDataList(7, designGoo.Value.Connections.Select(a => new ConnectionGoo(a.DeepClone())));
        DA.SetDataList(8, designGoo.Value.Qualities.Select(q => new QualityGoo(q.DeepClone())));
    }
}

public class KitComponent : SemioComponent
{
    public KitComponent()
        : base("Model Kit", "~Kit",
            "Construct, deconstruct or modify a kit.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");

    protected override Bitmap Icon => Resources.kit_modify_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "Ki?",
            "Optional kit to deconstruct or modify.",
            GH_ParamAccess.item);
        pManager[0].Optional = true;
        pManager.AddTextParameter("Name", "Na", "Name of the kit.", GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the kit.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the kit.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddTextParameter("Url", "Ur?", "Optional url of the kit.", GH_ParamAccess.item);
        pManager[4].Optional = true;
        pManager.AddTextParameter("Homepage", "Hp?", "Optional homepage of the kit.", GH_ParamAccess.item);
        pManager[5].Optional = true;
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "Types of the kit.", GH_ParamAccess.list);
        pManager[6].Optional = true;
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "Designs of the kit.", GH_ParamAccess.list);
        pManager[7].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "Ki",
            "Constructed or modified kit.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Description", "Dc?", "Optional description of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur?", "Optional url of the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Homepage", "Hp?", "Optional homepage of the kit.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty*", "Optional types of the kit.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "Dn*", "Optional designs of the kit.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var kitGoo = new KitGoo();
        var name = "";
        var description = "";
        var icon = "";
        var url = "";
        var homepage = "";
        var typeGoos = new List<TypeGoo>();
        var designGoos = new List<DesignGoo>();

        if (DA.GetData(0, ref kitGoo))
            kitGoo = kitGoo.Duplicate() as KitGoo;
        if (DA.GetData(1, ref name))
            kitGoo.Value.Name = name;
        if (DA.GetData(2, ref description))
            kitGoo.Value.Description = description;
        if (DA.GetData(3, ref icon))
            kitGoo.Value.Icon = icon;
        if (DA.GetData(4, ref url))
            kitGoo.Value.Url = url;
        if (DA.GetData(5, ref homepage))
            kitGoo.Value.Homepage = homepage;
        if (DA.GetDataList(6, typeGoos))
            kitGoo.Value.Types = typeGoos.Select(t => t.Value).ToList();
        if (DA.GetDataList(7, designGoos))
            kitGoo.Value.Designs = designGoos.Select(d => d.Value).ToList();

        var isValidInput = true;
        if (kitGoo.Value.Name == "")
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "A kit needs a name.");
            isValidInput = false;
        }

        if (!isValidInput) return;

        DA.SetData(0, kitGoo.Duplicate());
        DA.SetData(1, kitGoo.Value.Name);
        DA.SetData(2, kitGoo.Value.Description);
        DA.SetData(3, kitGoo.Value.Icon);
        DA.SetData(4, kitGoo.Value.Url);
        DA.SetData(5, kitGoo.Value.Homepage);
        DA.SetDataList(6, kitGoo.Value.Types.Select(t => new TypeGoo(t.DeepClone())));
        DA.SetDataList(7, kitGoo.Value.Designs.Select(d => new DesignGoo(d.DeepClone())));
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

public abstract class EngineComponent : SemioComponent
{
    protected EngineComponent(string name, string nickname, string description, string category, string subcategory)
        : base(name, nickname, description, category, subcategory)
    {
    }

    protected override void BeforeSolveInstance()
    {
        base.BeforeSolveInstance();
        try
        {
            var processes = Process.GetProcessesByName("semio-engine");
            IPGlobalProperties ipGlobalProperties = IPGlobalProperties.GetIPGlobalProperties();
            IPEndPoint[] tcpListeners = ipGlobalProperties.GetActiveTcpListeners();
            bool isSemioPortInUse = tcpListeners.Any(endpoint => endpoint.Port == 5052);
            if (processes.Length == 0 || !isSemioPortInUse)
            {
                var executableName = "semio-engine" +
                                     (Environment.OSVersion.Platform == PlatformID.Win32NT ? ".exe" : string.Empty);
                var path = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? string.Empty,
                    executableName);
                var engine = Process.Start(path);

                // lightweight way to kill child process when parent process is killed
                // https://stackoverflow.com/questions/3342941/kill-child-process-when-parent-process-is-killed#4657392
                AppDomain.CurrentDomain.DomainUnload += (s, e) =>
                {
                    engine.Kill();
                    engine.WaitForExit();
                };
                AppDomain.CurrentDomain.ProcessExit += (s, e) =>
                {
                    engine.Kill();
                    engine.WaitForExit();
                };
                AppDomain.CurrentDomain.UnhandledException += (s, e) =>
                {
                    engine.Kill();
                    engine.WaitForExit();
                };
            }
        }
        catch (Exception e)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, "semio-engine could not be started automatically.\n" + e.Message);
        }

    }
}

public class LoadKitComponent : EngineComponent
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
        pManager.AddParameter(new KitParam());
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
        if (!run) return;

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

        DA.SetData(0, new KitGoo(kit));
    }
}

public class CreateKitComponent : EngineComponent
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
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will take the current directory from which the Grasshopper script is executed.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Create the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the kit was created.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var kitGoo = new KitGoo();
        var path = "";
        var run = false;

        DA.GetData(0, ref kitGoo);
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

        var response = new Api().CreateLocalKit(path, kitGoo.Value);
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

public class DeleteKitComponent : EngineComponent
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

public class AddTypeComponent : EngineComponent
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

        if (DA.GetData(0, ref typeGoo))
            typeGoo = typeGoo.Duplicate() as TypeGoo;
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

public class AddDesignComponent : EngineComponent
{
    public AddDesignComponent()
        : base("Add Design", "+Dsn",
            "Add a design to a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("8B7AA946-0CB1-4CA8-A712-610B60425368");

    protected override Bitmap Icon => Resources.design_add_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Design to add to the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Add the design to the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the design was added to the kit.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designGoo = new DesignGoo();
        var path = "";
        var run = false;

        if (DA.GetData(0, ref designGoo))
            designGoo = designGoo.Duplicate() as DesignGoo;
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

        var response = new Api().AddDesignToLocalKit(path, designGoo.Value);
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

public class RemoveTypeComponent : EngineComponent
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
        pManager.AddTextParameter("Type Variant", "TyVn?",
            "Optional variant of the type to remove from the kit. No variant will remove the default variant.",
            GH_ParamAccess.item);
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
        var typeVariant = "";
        var path = "";
        var run = false;

        DA.GetData(0, ref typeName);
        DA.GetData(1, ref typeVariant);
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
            Name = typeName,
            Variant = typeVariant
        };
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

public class RemoveDesignComponent : EngineComponent
{
    public RemoveDesignComponent()
        : base("Remove Design", "-Dsn",
            "Remove a design from a kit.",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("9ECCE095-9D1E-4554-A3EB-1EAEEE2B12D5");

    protected override Bitmap Icon => Resources.design_remove_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Design Name", "DnNa",
            "Name of the design to remove from the kit.", GH_ParamAccess.item);
        pManager.AddTextParameter("Design Variant", "DnVn?",
            "Optional variant of the design to remove from the kit. No variant will remove the default variant.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Remove the design from the kit.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the design was removed from the kit.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designName = "";
        var designVariant = "";
        var path = "";
        var run = false;

        DA.GetData(0, ref designName);
        DA.GetData(1, ref designVariant);
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

        var design = new DesignId
        {
            Name = designName,
            Variant = designVariant
        };
        var response = new Api().RemoveDesignFromLocalKit(path, design);
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

public class SerializeDesignComponent : SemioComponent
{
    public SerializeDesignComponent()
        : base("Serialize Design", ">For",
            "Serialize a design.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");

    protected override Bitmap Icon => Resources.design_serialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Design to serialize.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized design.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designGoo = new DesignGoo();

        DA.GetData(0, ref designGoo);
        var text = designGoo.Value.Serialize();
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

public class DeserializeDesignComponent : SemioComponent
{
    public DeserializeDesignComponent()
        : base("Deserialize Design", "<For",
            "Deserialize a design.",
            "semio", "Scripting")
    {
    }

    public override Guid ComponentGuid => new("464D4D72-CFF1-4391-8C31-9E37EB9434C6");

    protected override Bitmap Icon => Resources.design_deserialize_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized design.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Deserialized design.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var base64Text = "";

        DA.GetData(0, ref base64Text);
        var textBytes = Convert.FromBase64String(base64Text);
        var text = Encoding.UTF8.GetString(textBytes);

        var design = text.Deserialize<Design>();

        DA.SetData(0, new DesignGoo(design));
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
            "Get a scene from a design.",
            "semio", "Viewing")
    {
    }

    public override Guid ComponentGuid => new("55F3BF32-3B4D-4355-BFAD-F3CA3847FC94");

    protected override Bitmap Icon => Resources.scene_get_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Design Name", "DnNa",
            "Name of design to convert to a scene.", GH_ParamAccess.item);
        pManager.AddTextParameter("Design Variant", "DnVn?",
            "Optional variant of the design to convert to a scene. No variant will convert the default variant.",
            GH_ParamAccess.item);
        pManager[1].Optional = true;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Convert the design to a scene.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new SceneParam(), "Scene", "Sc", "Scene.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designName = "";
        var designVariant = "";
        var path = "";
        var run = false;

        DA.GetData(0, ref designName);
        DA.GetData(1, ref designVariant);
        if (!DA.GetData(2, ref path))
            path = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(3, ref run);

        if (!run) return;

        var response = new Api().DesignToSceneFromLocalKit(path, new DesignId
        {
            Name = designName,
            Variant = designVariant
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
        pManager.AddTextParameter("Mimes", "Mm*", "Optional mimes of the representations in the scene.",
            GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+",
            "Representation of the objects of the scene.", GH_ParamAccess.list);
        pManager.AddPlaneParameter("Planes", "Pl+", "Planes of the objects of the scene.", GH_ParamAccess.list);
        pManager.AddTextParameter("Pieces Ids", "PcId+",
            "Ids of the pieces from the design that correspond to the objects of the scene.", GH_ParamAccess.list);
        pManager.AddTextParameter("Parents Pieces Ids", "PaPcId+",
            "Ids of the parent pieces from the design that correspond to the objects of the scene.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var sceneGoo = new SceneGoo();
        var lods = new List<string>();
        var tags = new List<string>();
        var mimes = new List<string>();

        DA.GetData(0, ref sceneGoo);
        DA.GetDataList(1, lods);
        DA.GetDataList(2, tags);
        DA.GetDataList(3, mimes);

        // filter the representations of the scene
        // if lods are used, only the representations with the specified lods are returned
        // if tags are used, each representations must have at least one of the specified tags
        // if mimes are used, only the representations with the specified mimes are returned
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
                    if (mimes.Count > 0)
                        if (!mimes.Contains(r.Mime))
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