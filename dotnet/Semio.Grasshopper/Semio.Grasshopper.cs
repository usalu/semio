#region License

//Semio.Grasshopper.cs
//2020-2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Affero General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Affero General Public License for more details.

//You should have received a copy of the GNU Affero General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion

#region TODOs

// TODO: Think of modelling components that are resilient to future schema changes.
// TODO: Refactor EngineComponent with GetInput and GetPersitanceInput etc. Very confusing. Probably no abstracting is better.
// TODO: GetProps and SetProps (includes children) is not consistent with the prop naming in python (does not include children).
// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();
// TODO: IsInvalid is used to check null state which is not clean.
// Think of a better way to handle this.
// The invalid check happen twice and code is duplicated.
// TODO: Figure out why cast from Piece to Text is not triggering the casts. ToString has somehow has precedence.
// TODO: NameM.ToLower() doesn't work for composite names. E.g. "DiagramPoint" -> "diagrampoint".
// TODO: Implement a status check and wait for the engine to be ready

#endregion

#region Usings

using System.Collections.Immutable;
using System.Diagnostics;
using System.Drawing;
using System.Reflection;
using System.Security.Cryptography;
using System.Text;
using FluentValidation;
using GH_IO.Serialization;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
using Grasshopper.Kernel.Types;
using Rhino;
using Rhino.Geometry;

#endregion

namespace Semio.Grasshopper;

#region Constants

public static class Constants
{
    public const string Category = Semio.Constants.Name;
    public const string Version = "5.0.0-beta";
}

#endregion

#region General

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => Semio.Constants.Name;
    public override Bitmap Icon => Resources.semio_24x24;
    public override Bitmap AssemblyIcon => Resources.semio_24x24;
    public override string Description => "semio within 🦗.";
    public override Guid Id => new("FE587CBF-5F7D-4091-AA6D-D9D30CF80B64");
    public override string Version => Constants.Version;
    public override string AuthorName => "Ueli Saluz";
    public override string AuthorContact => "ueli@semio-tech.org";
}

public class SemioCategoryIcon : GH_AssemblyPriority
{
    public override GH_LoadingInstruction PriorityLoad()
    {
        Instances.ComponentServer.AddCategoryIcon("semio", Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}

#endregion

#region Copilot

#region GraphQL

#endregion

#region Dictionary

//Symbol,Code,Abbreviation,Name,Description
//👥,Bs,Bas,Base,The shared base props for {{NAME}} models.
//🧲,Cd,Cnd,Connected,The connected side of the piece of the connection.
//🧲,Cg,Cng,Connecting,The connecting side of the piece of the connection.
//🖇️,Co,Con,Connection,A connection between two pieces in a design.
//🖇️,Co*,Cons,Connections,The optional connections of a design.
//⌚,CA,CAt,Created At,The time when the {{NAME}} was created.
//💬,Dc?,Dsc,Description,The optional human-readable description of the {{NAME}}.
//📖,Df,Def,Definition,The optional definition [ text | uri ] of the quality.
//✏️,Dg,Dgm,Diagram,The diagram of the design.
//📁,Di?,Dir,Directory,The optional directory where to find the kit.
//🏅,Dl,Dfl,Default,Whether it is the default representation of the type. There can be only one default representation per type.
//➡️,Dr,Drn,Direction,The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.
//🏙️,Dn,Dsn,Design,A design is a collection of pieces that are connected.
//🏙️,Dn*,Dsns,Designs,The optional designs of the kit.
//📺,DP,DPt,Diagram Point,A 2d-point (xy) of floats in the diagram. One unit is equal the width of a piece icon.
//🚌,Dt,DTO,Data Transfer Object, The Data Transfer Object (DTO) base of the {{NAME}}.
//🪣,Em,Emp,Empty,Empty all props and children of the {{NAME}}.
//▢,En,Ent,Entity,An entity is a collection of properties and children.
//🔑,FK,FKy,Foreign Key, The foreign primary key of the parent {{PARENT_NAME}} of the {{NAME}} in the database.
//↕️,Gp?,Gap,Gap,The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. 
//🆔,GI,GID,Globally Unique Identifier,A Globally Unique Identifier (GUID) of the entity.
//👪,Gr,Grp,Group,The group of the locator.
//🏠,Hp?,Hmp,Homepage,The optional url of the homepage of the kit.
//🪙,Ic?,Ico,Icon,The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. {{NAME}}.
//🆔,Id,Id,Identifier,The local identifier of the {{NAME}} within the {{PARENT_NAME}}.
//🆔,Id?,Id,Identifier,The optional local identifier of the {{NAME}} within the {{PARENT_NAME}}. No id means the default {{NAME}}.
//🪪,Id,Id,Identifier,The props to identify the {{NAME}} within the parent {{PARENT_NAME}}.
//↘️,In,Inp,Input,The input for a {{NAME}}.
//🗃️,Kt,Kit,Kit,A kit is a collection of designs that use types.
//🗺️,Lc,Loc,Locator,A locator is machine-readable metadata for grouping ports and provides a mechanism to easily switch between ports based on individual locators.
//🗺️,Lc*,Locs,Locators,The optional machine-readable locators of the port. Every port should have a unique set of locators.
//🔍,Ld?,Lod,Level of Detail,The optional Level of Detail/Development/Design (LoD) of the representation. No lod means the default lod.
//📛,Na,Nam,Name,The name of the {{NAME}}.
//✉️,Mm,Mim,Mime,The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.
//⌱,Og,Org,Origin,The origin of the plane.
//↗️,Ou,Out,Output,The output for a {{NAME}}.
//👪,Pa,Par,Parent,The parent of {{NAME}}.
//⚒️,Pr,Prs,Parse,Parse the {{NAME}} from an input.
//🔢,Pl,Plu,Plural,The plural of the singular of the entity name.
//⭕,Pc,Pce,Piece,A piece is a 3d-instance of a type in a design.
//⭕,Pc?,Pces,Pieces,The optional pieces of the design.
//🔑,PK,PKy,Primary Key, The {{PROP_NAME}} is the primary key of the {{NAME}} in the database.
//🔌,Po,Por,Port,A port is a connection point (with a direction) of a type.
//🔌,Po+,Pors,Ports,The ports of the type.
//🎫,Pp,Prp,Props,The props are all values of an entity without its children.
//◳,Pn,Pln,Plane,A plane is an origin (point) and an orientation (x-axis and y-axis).
//◳,Pn?,Pln,Plane,The optional plane of the piece. When pieces are connected only one piece can have a plane.
//✖️,Pt,Pnt,Point,A 3d-point (xyz) of floating point numbers.
//✖️,Pt,Pnt,Point,The connection point of the port that is attracted to another connection point.
//📏,Ql,Qal,Quality,A quality is a named value with a unit and a definition.
//📏,Ql*,Qals,Qualities,The optional machine-readable qualities of the  {{NAME}}.
//🍾,Rl,Rel,Release,The release of the engine that created this database.
//☁️,Rm?,Rmt,Remote,The optional Unique Resource Locator (URL) where to fetch the kit remotely.
//💾,Rp,Rep,Representation,A representation is a link to a resource that describes a type for a certain level of detail and tags.
//🔄,Rt?,Rot,Rotation,The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.
//🧱,Sd,Sde,Side,A side of a piece in a connection.
//↔️,Sf,Sft,Shift,The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece.
//📌,SG?,SGr,Subgroup,The optional sub-group of the locator. No sub-group means true.
//✅,Su,Suc,Success,{{NAME}} was successful.
//🏷️,Tg*,Tags,Tags,The optional tags to group representations. No tags means default.
//↗️,Tl?,Tlt,Tilt,The optional horizontal tilt perpendicular to the port direction (applied after rotation) between the connected and the connecting piece in degrees.
//▦,Tf,Trf,Transform,A 4x4 translation and rotation transformation matrix (no scaling or shearing).
//🧩,Ty,Typ,Type,A type is a reusable element that can be connected with other types over ports.
//🧩,Ty,Typ,Type,The type-related information of the side.
//🧩,Ty*,Typs,Types,The optional types of the kit.
//🔗,Ur,Url,Unique Resource Locator,The Unique Resource Locator (URL) to the resource of the representation.
//Ⓜ️,Ut,Unt,Unit,The length unit for all distance-related information of the {{PARENT_NAME}}.
//Ⓜ️,Ut,Unt,Unit,The optional unit of the value of the quality.
//🔄,Up,Upd,Update,Update the props of the {{NAME}}. Optionally empty the {{NAME}} before.
//🔀,Vn?,Vnt,Variant,The optional variant of the {{PARENT_NAME}}. No variant means the default variant. 
//➡️,Vc,Vec,Vector,A 3d-vector (xyz) of floating point numbers.
//🔀,Ve,Ver,Version,The optional version of the kit. No version means the latest version.
//🛂,Vd,Vld,Validate,Check if the {{NAME}} is valid.
//🏷️,Vl,Val,Value,The value of the tag.
//🔢,Vl?,Val,Value,The optional value [ text | url ] of the quality. No value is equivalent to true for the name.
//🔀,Vn?,Vnt,Variant,The optional variant of the {{NAME}}. No variant means the default variant.
//🏁,X,X,X,The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
//🎚️,X,X,X,The x-coordinate of the point.
//➡️,XA,XAx,XAxis,The x-axis of the plane.
//🏁,Y,Y,Y,The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
//🎚️,Y,Y,Y,The y-coordinate of the point.
//➡️,YA,YAx,YAxis,The y-axis of the plane.
//🏁,Z,Z,Z,The z-coordinate of the screen point.
//🎚️,Z,Z,Z,The z-coordinate of the point.
//➡️,ZA,ZAx,ZAxis,The z-axis of the plane.

#endregion

#endregion

#region Utility

public static class Utility
{
    public static bool IsValidLengthUnitSystem(string unit)
    {
        return new[] { "nm", "mm", "cm", "dm", "m", "km", "µin", "in", "ft", "yd" }.Contains(unit);
    }

    public static string LengthUnitSystemToAbbreviation(UnitSystem unitSystem)
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
            _ => "unsupported length unit system"
        };
        if (IsValidLengthUnitSystem(unit) == false)
            throw new ArgumentException("Invalid length unit system", nameof(unitSystem));
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

    public static Plane ComputeChildPlane(Plane parentPlane, Point parentPoint, Vector parentDirection,
        Point childPoint, Vector childDirection, float rotation, float tilt, float gap, float shift)
    {
        var parentPointR = new Vector3d(parentPoint.Convert());
        var parentDirectionR = parentDirection.Convert();
        var revertedChildPointR = new Vector3d(childPoint.Convert());
        revertedChildPointR.Reverse();
        var gapDirectionR = new Vector3d(parentDirectionR);
        var reverseChildDirectionR = childDirection.Convert();
        reverseChildDirectionR.Reverse();
        var rotationRad = RhinoMath.ToRadians(rotation);
        var tiltRad = RhinoMath.ToRadians(tilt);

        // orient

        // If directions are same, then there are infinite solutions.
        var areDirectionsSame = parentDirectionR.IsParallelTo(childDirection.Convert(), Semio.Constants.Tolerance) == 1;

        // Rhino tends to pick the "wrong" direction when the vectors are parallel.
        // E.g when z=0 it picks to flip the object around the y-axis instead of a rotation around the z-axis.
        Transform directionT;
        if (areDirectionsSame)
        {
            // Idea taken from: // https://github.com/dfki-ric/pytransform3d/blob/143943b028fc776adfc6939b1d7c2c6edeaa2d90/pytransform3d/rotations/_utils.py#L253
            if (Math.Abs(parentDirectionR.Z) < Semio.Constants.Tolerance)
                directionT = Transform.Rotation(RhinoMath.ToRadians(180), Vector3d.ZAxis, new Point3d());
            else
                directionT = Transform.Rotation(RhinoMath.ToRadians(180),
                    Vector3d.CrossProduct(Vector3d.ZAxis, parentDirectionR), new Point3d());
        }
        else
        {
            directionT = Transform.Rotation(reverseChildDirectionR, parentDirectionR, new Point3d());
        }

        var tiltAxisRotation = Transform.Rotation(Vector3d.YAxis, parentDirectionR, new Point3d());
        var tiltAxis = Vector3d.XAxis;
        tiltAxis.Transform(tiltAxisRotation);

        var gapDirection = gapDirectionR;

        var orientationT = directionT;

        var rotateT = Transform.Rotation(-rotationRad, parentDirectionR, new Point3d());
        orientationT = rotateT * directionT;
        tiltAxis.Transform(rotateT);
        gapDirection.Transform(rotateT);

        var tiltT = Transform.Rotation(tiltRad, tiltAxis, new Point3d());
        orientationT = tiltT * orientationT;
        gapDirection.Transform(tiltT);

        // move

        var centerChild = Transform.Translation(revertedChildPointR);
        var moveToParent = Transform.Translation(parentPointR);
        var transform = orientationT * centerChild;


        var gapTransform = Transform.Translation(gapDirection * gap);
        var shiftDirection = new Vector3d(tiltAxis) * shift;
        var shiftTransform = Transform.Translation(shiftDirection);
        var translation = gapTransform * shiftTransform;

        transform = translation * transform;

        transform = moveToParent * transform;
        var childPlaneR = Rhino.Geometry.Plane.WorldXY;
        childPlaneR.Transform(transform);

        // to parent

        var parentPlaneR = parentPlane.Convert();
        var parentPlaneT = Transform.PlaneToPlane(Rhino.Geometry.Plane.WorldXY, parentPlaneR);
        childPlaneR.Transform(parentPlaneT);

        return childPlaneR.Convert();
    }
}

#endregion

#region Converters

public static class RhinoConverter
{
    public static object Convert(this object value)
    {
        return value;
    }

    public static string Convert(this string value)
    {
        return value;
    }

    public static int Convert(this int value)
    {
        return value;
    }

    public static float Convert(this double value)
    {
        return (float)value;
    }

    public static Point3d Convert(this Point point)
    {
        return new Point3d(point.X, point.Y, point.Z);
    }

    public static Point Convert(this Point3d point)
    {
        return new Point
        {
            X = (float)point.X,
            Y = (float)point.Y,
            Z = (float)point.Z
        };
    }

    public static Vector3d Convert(this Vector vector)
    {
        return new Vector3d(vector.X, vector.Y, vector.Z);
    }

    public static Vector Convert(this Vector3d vector)
    {
        return new Vector
        {
            X = (float)vector.X,
            Y = (float)vector.Y,
            Z = (float)vector.Z
        };
    }

    public static Rhino.Geometry.Plane Convert(this Plane plane)
    {
        return new Rhino.Geometry.Plane(
            new Point3d(plane.Origin.X, plane.Origin.Y, plane.Origin.Z),
            new Vector3d(plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z),
            new Vector3d(plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z)
        );
    }

    public static Plane Convert(this Rhino.Geometry.Plane plane)
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

public abstract class ModelGoo<T> : GH_Goo<T> where T : Model<T>, new()
{
    public ModelGoo()
    {
        Value = new T();
    }

    public ModelGoo(T value)
    {
        Value = value;
    }

    public override bool IsValid { get; }

    public override string TypeName => typeof(T).Name;

    public override string TypeDescription =>
        ((ModelAttribute)Attribute.GetCustomAttribute(typeof(T), typeof(ModelAttribute))).Description;

    public override IGH_Goo Duplicate()
    {
        var duplicate = (ModelGoo<T>)Activator.CreateInstance(GetType());
        duplicate.Value = Value.DeepClone();
        return duplicate;
    }

    public override string ToString()
    {
        if (Value == null)
            return null;
        return Value.ToString();
    }

    public override bool Write(GH_IWriter writer)
    {
        writer.SetString(typeof(T).Name, Value.Serialize());
        return base.Write(writer);
    }

    public override bool Read(GH_IReader reader)
    {
        Value = reader.GetString(typeof(T).Name).Deserialize<T>();
        return base.Read(reader);
    }
}

public class RepresentationGoo : ModelGoo<Representation>
{
    public RepresentationGoo()
    {
    }

    public RepresentationGoo(Representation value) : base(value)
    {
    }
}

public class LocatorGoo : ModelGoo<Locator>
{
    public LocatorGoo()
    {
    }

    public LocatorGoo(Locator value) : base(value)
    {
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

public class PortGoo : ModelGoo<Port>
{
    public PortGoo()
    {
    }

    public PortGoo(Port value) : base(value)
    {
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Plane)))
        {
            object ptr = new GH_Plane(Utility.GetPlaneFromYAxis(Value.Direction.Convert(), 0, Value.Point.Convert()));
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
            Value.Point = plane.Origin.Convert();
            Value.Direction = plane.YAxis.Convert();

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
}

public class QualityGoo : ModelGoo<Quality>
{
    public QualityGoo()
    {
    }

    public QualityGoo(Quality value) : base(value)
    {
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

public class AuthorGoo : ModelGoo<Author>
{
    public AuthorGoo()
    {
    }

    public AuthorGoo(Author value) : base(value)
    {
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
        {
            object ptr = new GH_String(Value.Email);
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
            Value = new Author
            {
                Email = str
            };
            return true;
        }

        return false;
    }
}

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo()
    {
    }

    public TypeGoo(Type value) : base(value)
    {
    }
}

public class DiagramPointGoo : ModelGoo<DiagramPoint>
{
    public DiagramPointGoo()
    {
    }

    public DiagramPointGoo(DiagramPoint value) : base(value)
    {
    }

    public override bool CastTo<Q>(ref Q target)
    {
        if (typeof(Q).IsAssignableFrom(typeof(GH_Point)))
        {
            if (Value == null)
                return false;
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
            Value = new DiagramPoint
            {
                X = (float)point.X,
                Y = (float)point.Y
            };
            return true;
        }

        return false;
    }
}

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo()
    {
    }

    public PieceGoo(Piece value) : base(value)
    {
    }

    // TODO: Figure out why cast from Piece to Text is not triggering the casts. ToString has somehow has precedence.
    //public override bool CastTo<Q>(ref Q target)
    //{
    //    if (typeof(Q).IsAssignableFrom(typeof(GH_String)))
    //    {
    //        object ptr = new GH_String(Value.Id);
    //        target = (Q)ptr;
    //        return true;
    //    }

    //    return false;
    //}

    //public override bool CastFrom(object source)
    //{
    //    if (source == null) return false;

    //    string str;
    //    if (GH_Convert.ToString(source, out str, GH_Conversion.Both))
    //    {
    //        Value = new Piece
    //        {
    //            Id = str
    //        };
    //        return true;
    //    }
    //    return false;
    //}
}

public class ConnectionGoo : ModelGoo<Connection>
{
    public ConnectionGoo()
    {
    }

    public ConnectionGoo(Connection value) : base(value)
    {
    }
}

public class DesignGoo : ModelGoo<Design>
{
    public DesignGoo()
    {
    }

    public DesignGoo(Design value) : base(value)
    {
    }
}

public class KitGoo : ModelGoo<Kit>
{
    public KitGoo()
    {
    }

    public KitGoo(Kit value) : base(value)
    {
    }
}

#endregion

#region Params

public abstract class ModelParam<T, U> : GH_PersistentParam<T> where T : ModelGoo<U> where U : Model<U>, new()
{
    internal ModelParam() : base(typeof(U).Name,
        ((ModelAttribute)Attribute.GetCustomAttribute(typeof(U), typeof(ModelAttribute))).Code,
        ((ModelAttribute)Attribute.GetCustomAttribute(typeof(U), typeof(ModelAttribute))).Description,
        Constants.Category,
        "Params")
    {
    }

    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{typeof(U).Name.ToLower()}_24x24");

    protected override GH_GetterResult Prompt_Singular(ref T value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<T> values)
    {
        throw new NotImplementedException();
    }
}

public class RepresentationParam : ModelParam<RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("895BBC91-851A-4DFC-9C83-92DFE90029E8");
}

public class LocatorParam : ModelParam<LocatorGoo, Locator>
{
    public override Guid ComponentGuid => new("DBE104DA-63FA-4C68-8D41-834DD962F1D7");
}

public class PortParam : ModelParam<PortGoo, Port>
{
    public override Guid ComponentGuid => new("96775DC9-9079-4A22-8376-6AB8F58C8B1B");
}

public class QualityParam : ModelParam<QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("431125C0-B98C-4122-9598-F72714AC9B94");
}

public class AuthorParam : ModelParam<AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("9F52380B-1812-42F7-9DAD-952C2F7A635A");
}

public class TypeParam : ModelParam<TypeGoo, Type>
{
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
}

public class DiagramPointParam : ModelParam<DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("4685CCE8-C629-4638-8DF6-F76A17571841");
}

public class PieceParam : ModelParam<PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("76F583DC-4142-4346-B1E1-6C241AF26086");
}

public class ConnectionParam : ModelParam<ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");
}

public class DesignParam : ModelParam<DesignGoo, Design>
{
    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");
}

public class KitParam : ModelParam<KitGoo, Kit>
{
    public override Guid ComponentGuid => new("BA9F161E-AFE3-41D5-8644-964DD20B887B");
}

#endregion

#region Components

public abstract class Component : GH_Component
{
    public Component(string name, string nickname, string description, string subcategory) : base(
        name, nickname, description, Constants.Category, subcategory)
    {
    }
}

#region Scripting

public abstract class ScriptingComponent : Component
{
    public ScriptingComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Scripting")
    {
    }
}

public class EncodeTextComponent : ScriptingComponent
{
    public EncodeTextComponent()
        : base("Encode Text", ">Txt", "Encode a text.")
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
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        DA.SetData(0, Semio.Utility.Encode(text));
    }
}

public class DecodeTextComponent : ScriptingComponent
{
    public DecodeTextComponent()
        : base("Decode Text", "<Txt", "Decode a text.")
    {
    }

    public override Guid ComponentGuid => new("E7158D28-87DE-493F-8D78-923265C3E211");

    protected override Bitmap Icon => Resources.decode_24x24;

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Encoded Text", "En", "Encoded text to decode.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Decoded text.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var encodedText = "";
        DA.GetData(0, ref encodedText);
        DA.SetData(0, Semio.Utility.Decode(encodedText));
    }
}

#region Serialize

public abstract class SerializeComponent<T, U, V> : ScriptingComponent
    where T : ModelParam<U, V>, new() where U : ModelGoo<V>, new() where V : Model<V>, new()

{
    public static readonly string NameM;
    public static readonly ModelAttribute ModelM;

    static SerializeComponent()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMetaGrasshopper = Meta.Goo;

        NameM = typeof(V).Name;
        ModelM = Semio.Meta.Model[NameM];
    }

    protected SerializeComponent() : base($"Serialize {NameM}", $">{ModelM.Abbreviation}",
        $"Serialize a {NameM.ToLower()}.")
    {
    }

    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{NameM.ToLower()}_serialize_24x24");
    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new T(), NameM, ModelM.Code,
            $"The {NameM.ToLower()} to serialize.", GH_ParamAccess.item);
        pManager.AddTextParameter("Indent", "In?", $"The optional indent unit for the serialized {NameM.ToLower()}. Empty text for no indent or spaces or tabs", GH_ParamAccess.item, "");
        pManager[1].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", "Text of serialized " + NameM + ".", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var goo = new U();
        var indent = "";
        DA.GetData(0, ref goo);
        DA.GetData(1, ref indent);
        var text = goo.Value.Serialize(indent);
        DA.SetData(0, text);
    }
}

public class
    SerializeRepresentationComponent : SerializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("AC6E381C-23EE-4A81-BE0F-3523AEE32046");
}

public class SerializeLocatorComponent : SerializeComponent<LocatorParam, LocatorGoo, Locator>
{
    public override Guid ComponentGuid => new("7AFC411B-57D4-4B36-982C-495E14E7520E");
}

public class SerializePortComponent : SerializeComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("1A29F6ED-464D-490F-B072-3412B467F1B5");
}

public class SerializeQualityComponent : SerializeComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");
}

public class SerializeAuthorComponent : SerializeComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("99130A53-4FC1-4E64-9A46-2ACEC4634878");
}

public class SerializeTypeComponent : SerializeComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");
}

public class SerializeDiagramPointComponent : SerializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("EDD83721-D2BD-4CF1-929F-FBB07F0A6A99");
}

public class SerializePieceComponent : SerializeComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("A4EDA838-2246-4617-8298-9585ECFE00D9");
}

public class SerializeConnectionComponent : SerializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("93FBA84E-79A1-4E32-BE61-A925F476DD60");
}

public class SerializeDesignComponent : SerializeComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");
}

public class SerializeKitComponent : SerializeComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("78202ACE-A876-45AF-BA72-D1FC00FE4165");
}

#endregion

#region Deserialize

public abstract class DeserializeComponent<T, U, V> : ScriptingComponent
    where T : ModelParam<U, V>, new() where U : ModelGoo<V>, new() where V : Model<V>, new()

{
    public static readonly string NameM;
    public static readonly ModelAttribute ModelM;

    static DeserializeComponent()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMetaGrasshopper = Meta.Goo;

        NameM = typeof(V).Name;
        ModelM = Semio.Meta.Model[NameM];
    }

    protected DeserializeComponent() : base($"Deserialize {NameM}", $"<{ModelM.Abbreviation}",
        $"Deserialize a {NameM.ToLower()}.")
    {
    }

    protected override Bitmap Icon =>
        (Bitmap)Resources.ResourceManager.GetObject($"{NameM.ToLower()}_deserialize_24x24");

    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Text", "Tx", $"Text of serialized {NameM}.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new T(), NameM, ModelM.Code,
            $"Deserialized {NameM}.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var text = "";
        DA.GetData(0, ref text);
        var value = text.Deserialize<V>();
        var goo = new U();
        goo.Value = value;
        DA.SetData(0, goo);
    }
}

public class
    DeserializeRepresentationComponent : DeserializeComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("B8ADAF54-3A91-402D-9542-A288D935015F");
}

public class DeserializeLocatorComponent : DeserializeComponent<LocatorParam, LocatorGoo, Locator>
{
    public override Guid ComponentGuid => new("F3501014-D011-4421-9750-861B6479C83C");
}

public class DeserializePortComponent : DeserializeComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("3CEB0315-5A51-4072-97A7-D8B1B63FEF31");
}

public class DeserializeQualityComponent : DeserializeComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("AECB1169-EB65-470F-966E-D491EB46A625");
}

public class DeserializeAuthorComponent : DeserializeComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("DDC0A2EC-4BAD-4FFE-B3A6-F9644C8B0072");
}

public class DeserializeTypeComponent : DeserializeComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("F21A80E0-2A62-4BFD-BC2B-A04363732F84");
}

public class DeserializeDiagramPointComponent : DeserializeComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("7FBEECE1-ECAC-4AC1-8DAF-C659A9B6238C");
}

public class DeserializePieceComponent : DeserializeComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("1FB7F2FB-DCE2-4666-91B5-54DF6B6D9FA4");
}

public class DeserializeConnectionComponent : DeserializeComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("41C33A9F-15AC-4CD0-8A9D-4A75CE599282");
}

public class DeserializeDesignComponent : DeserializeComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("464D4D72-CFF1-4391-8C31-9E37EB9434C6");
}

public class DeserializeKitComponent : DeserializeComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("79AF9C1D-2B96-4D03-BDD9-C6514DA63E70");
}

#endregion

#endregion

#region Diagram

public class DrawDiagramComponent : Component
{
    public DrawDiagramComponent()
        : base("Draw Diagram", ":Dgm", "Draw the diagram from a design.", "Display")
    {
    }

    public override Guid ComponentGuid => new("C53A0CC8-6DD7-415E-A20A-C5887CBE0DB9");

    protected override Bitmap Icon => Resources.diagram_draw_24x24;

    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam());
        pManager.AddParameter(new TypeParam(), "Types", "Ty+",
            "Types that are used by the pieces in the design.", GH_ParamAccess.list);
        pManager.AddTextParameter("Uri", "Ur?",
            "Optional Unique Resource Identifier (URI) of the kit. This can be an absolute path to a local kit or a url to a remote kit.\n" +
            "If none is provided, it will try to see if the Grasshopper script is executed inside a local kit.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "True to create the diagram of the design.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddTextParameter("Scalable Vector Graphics", "SVG",
            "The diagram as a Scalable Vector Graphics (SVG).", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designGoo = new DesignGoo();
        var typesGoos = new List<TypeGoo>();
        var uri = "";
        var run = false;

        DA.GetData(0, ref designGoo);
        DA.GetDataList(1, typesGoos);
        if (!DA.GetData(2, ref uri))
            uri = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        DA.GetData(3, ref run);
        if (!run)
            return;
        var design = designGoo.Value;
        var types = typesGoos.Select(t => t.Value).ToArray();
        var svg = design.Diagram(types, Utility.ComputeChildPlane, uri);
        DA.SetData(0, svg);
    }
}

#endregion

#region Util

public class FlattenDesignComponent : Component
{
    public FlattenDesignComponent()
        : base("Flatten Design", "FltDsn", "Flatten a design.", "Util")
    {
    }

    public override Guid ComponentGuid => new("434144EA-2AFB-4D39-9F75-BB77A9223595");

    protected override Bitmap Icon => Resources.design_flatten_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Design to flatten.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty+",
            "Types that are used by the pieces in the design.", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dn",
            "Flat Design with no connections.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var designGoo = new DesignGoo();
        var typesGoos = new List<TypeGoo>();
        DA.GetData(0, ref designGoo);
        DA.GetDataList(1, typesGoos);
        var design = designGoo.Value;
        var types = typesGoos.Select(t => t.Value).ToArray();
        var flatDesign = design.DeepClone().Flatten(types, Utility.ComputeChildPlane);
        DA.SetData(0, new DesignGoo(flatDesign));
    }
}

public class ConvertUnitComponent : Component
{
    public ConvertUnitComponent()
        : base("Convert Unit", "CnvUnt", "Convert a unit.", "Util")
    {
    }

    public override Guid ComponentGuid => new("4EEB48B6-39A2-4FE1-B83F-6755EE355FF5");

    protected override Bitmap Icon => Resources.unit_convert_24x24;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddNumberParameter("Value", "Vl", "Value to convert.", GH_ParamAccess.item, 1);
        pManager.AddTextParameter("From Unit", "FU", "Unit to convert from.", GH_ParamAccess.item);
        pManager.AddTextParameter("To Unit", "TU", "Unit to convert to.", GH_ParamAccess.item);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddNumberParameter("Converted Value", "CV", "Converted value.", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var value = 0.0;
        var from = "";
        var to = "";
        DA.GetData(0, ref value);
        DA.GetData(1, ref from);
        DA.GetData(2, ref to);
        var convertedValue = Semio.Utility.Units.Convert((float)value, from, to);
        DA.SetData(0, (double)convertedValue);
    }
}

//public class UpdateComponents : Component
//{
//    public UpdateComponents()
//    : base("Update Components", "↑Cmps", "Update all components.", "Util")
//    {
//    }

//    public override Guid ComponentGuid => new("51AC98FB-167F-41EC-9BBA-867A0B3F9E0A");

//    protected override Bitmap Icon => Resources.components_update_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddBooleanParameter("Update", "Up", "Update all components.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddBooleanParameter("Updated", "Upd", "True if components were updated.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var update = false;
//        DA.GetData(0, ref update);
//        if (update)
//        {
//            foreach (var obj in Instances.ActiveCanvas.Document.Objects)
//            {
//                if (obj is GH_Component component)
//                    component.ExpireSolution(true);
//            }
//        }

//        DA.SetData(0, update);
//    }
//}

#endregion

#region Modelling

public abstract class ModelComponent<T, U, V> : Component
    where T : ModelParam<U, V> where U : ModelGoo<V> where V : Model<V>, new()
{
    public static readonly string NameM;
    public static readonly System.Type TypeM;
    public static readonly System.Type GooM;
    public static readonly System.Type ParamM;
    public static readonly ModelAttribute ModelM;
    public static readonly ImmutableArray<PropertyInfo> PropertyM;
    public static readonly ImmutableArray<PropAttribute> PropM;
    public static readonly ImmutableArray<bool> IsPropertyList;
    public static readonly ImmutableArray<bool> IsPropertyMapped;
    public static readonly ImmutableArray<System.Type> PropertyItemType;
    public static readonly ImmutableArray<bool> IsPropertyModel;
    public static readonly ImmutableArray<System.Type> PropertyGooM;
    public static readonly ImmutableArray<System.Type> PropertyParamM;
    public static readonly ImmutableArray<System.Type> PropertyItemGoo;

    static ModelComponent()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMetaGrasshopper = Meta.Goo;

        NameM = typeof(V).Name;
        TypeM = Semio.Meta.Type[NameM];
        GooM = Meta.Goo[NameM];
        ParamM = Meta.Param[NameM];
        ModelM = Semio.Meta.Model[NameM];
        PropertyM = Semio.Meta.Property[NameM];
        PropM = Semio.Meta.Prop[NameM];
        IsPropertyList = Semio.Meta.IsPropertyList[NameM];
        IsPropertyMapped = Meta.IsPropertyMapped[NameM];
        PropertyItemType = Semio.Meta.PropertyItemType[NameM];
        PropertyItemGoo = Meta.PropertyItemGoo[NameM];
        IsPropertyModel = Semio.Meta.IsPropertyModel[NameM];
        PropertyGooM = Meta.PropertyGoo[NameM];
        PropertyParamM = Meta.PropertyParam[NameM];
    }

    protected ModelComponent() : base($"Model {NameM}", $"~{ModelM.Abbreviation}",
        $"Construct, deconstruct or modify {Semio.Utility.Grammar.GetArticle(NameM)} {NameM.ToLower()}", "Modelling")
    {
    }

    protected override Bitmap Icon =>
        (Bitmap)Resources.ResourceManager.GetObject($"{typeof(V).Name.ToLower()}_modify_24x24");

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected virtual void AddModelProps(dynamic pManager)
    {
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var propAttribute = PropM[i];
            var param = (IGH_Param)Activator.CreateInstance(PropertyParamM[i]);
            pManager.AddParameter(param, property.Name, propAttribute.Code, propAttribute.Description,
                IsPropertyList[i] ? GH_ParamAccess.list : GH_ParamAccess.item);
        }
    }

    protected void AddModelParameters(dynamic pManager, bool isOutput = false)
    {
        var modelParam = (IGH_Param)Activator.CreateInstance(ParamM);
        var description = isOutput
            ? $"The constructed or modified {NameM.ToLower()}."
            : $"The optional {NameM.ToLower()} to deconstruct or modify.";
        pManager.AddParameter(modelParam, NameM, isOutput ? ModelM.Code : ModelM.Code + "?",
            description, GH_ParamAccess.item);
        pManager.AddBooleanParameter(isOutput ? "Valid" : "Validate", "Vd?",
            isOutput
                ? $"True if the {NameM.ToLower()} is valid. Null if no validation was performed."
                : $"Whether the {NameM.ToLower()} should be validated.", GH_ParamAccess.item);

        AddModelProps(pManager);

        if (!isOutput)
            for (var i = 0; i < pManager.ParamCount; i++)
                ((GH_InputParamManager)pManager)[i].Optional = true;
    }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        AddModelParameters(pManager);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        AddModelParameters(pManager, true);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        dynamic modelGoo = Activator.CreateInstance(GooM);
        var validate = false;
        if (DA.GetData(0, ref modelGoo))
            modelGoo = modelGoo.Duplicate();
        DA.GetData(1, ref validate);
        GetProps(DA, modelGoo);

        modelGoo.Value = ProcessModel(modelGoo.Value);

        if (validate)
        {
            var (isValid, errors) = ((bool, List<string>))modelGoo.Value.Validate();
            foreach (var error in errors)
                AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);
            DA.SetData(1, isValid);
        }

        DA.SetData(0, modelGoo.Duplicate());
        SetData(DA, modelGoo);
    }

    protected virtual void GetProps(IGH_DataAccess DA, dynamic modelGoo)
    {
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var isList = IsPropertyList[i];
            var itemType = PropertyItemType[i];
            dynamic gooValue = Activator.CreateInstance(PropertyGooM[i]);
            var value = gooValue;
            bool hasInput = isList ? DA.GetDataList(i + 2, value) : DA.GetData(i + 2, ref value);
            if (hasInput)
            {
                if (isList)
                {
                    var listType = typeof(List<>).MakeGenericType(itemType);
                    dynamic list = Activator.CreateInstance(listType);
                    foreach (var item in gooValue)
                        list.Add(itemType == typeof(string) || itemType == typeof(int) || itemType == typeof(float)
                            ? item.Value
                            : item.Value.DeepClone());

                    value = list;
                    property.SetValue(modelGoo.Value, value);
                }
                else
                {
                    property.SetValue(modelGoo.Value, RhinoConverter.Convert(value.Value));
                }
            }
        }
    }

    protected virtual void SetData(IGH_DataAccess DA, dynamic modelGoo) // TODO: Check if dynamic is necessary
    {
        for (var i = 0; i < PropertyM.Length; i++)
        {
            var property = PropertyM[i];
            var isList = IsPropertyList[i];
            var isPropertyModel = IsPropertyModel[i];
            var isPropertyMapped = IsPropertyMapped[i];
            var value = property.GetValue(modelGoo.Value);

            if (value == null)
                return;

            if (isList)
            {
                if (isPropertyModel)
                {
                    dynamic list = Activator.CreateInstance(PropertyGooM[i]);
                    foreach (var item in value)
                    {
                        var itemGoo = Activator.CreateInstance(PropertyItemGoo[i], item.DeepClone());
                        list.Add(itemGoo);
                    }

                    value = list;
                }
            }
            else if (isPropertyModel)
            {
                if (isPropertyMapped)
                {
                    var convertMethod =
                        typeof(RhinoConverter).GetMethod("Convert", new System.Type[] { value.GetType() });
                    value = convertMethod.Invoke(null, new[] { value });
                }
                else
                {
                    value = Activator.CreateInstance(PropertyItemGoo[i], value.DeepClone());
                }
            }

            if (isList)
                DA.SetDataList(i + 2, value);
            else
                DA.SetData(i + 2, value);
        }
    }

    protected virtual V ProcessModel(V model)
    {
        return model;
    }
}

public class RepresentationComponent : ModelComponent<RepresentationParam, RepresentationGoo, Representation>
{
    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

    protected override Representation ProcessModel(Representation model)
    {
        if (model.Mime == "")
            model.Mime = Semio.Utility.ParseMimeFromUrl(model.Url);
        model.Url = model.Url.Replace('\\', '/');
        return model;
    }
}

public class LocatorComponent : ModelComponent<LocatorParam, LocatorGoo, Locator>
{
    public override Guid ComponentGuid => new("2552DB71-8459-4DB5-AD66-723573E771A2");
}

public class PortComponent : ModelComponent<PortParam, PortGoo, Port>
{
    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");
}

public class QualityComponent : ModelComponent<QualityParam, QualityGoo, Quality>
{
    public override Guid ComponentGuid => new("51146B05-ACEB-4810-AD75-10AC3E029D39");
}

public class AuthorComponent : ModelComponent<AuthorParam, AuthorGoo, Author>
{
    public override Guid ComponentGuid => new("5143ED92-0A2C-4D0C-84ED-F90CC8450894");
}

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");

    protected override Type ProcessModel(Type type)
    {
        if (type.Unit == "")
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                type.Unit = Utility.LengthUnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                type.Unit = "m";
            }

        type.Icon = type.Icon.Replace('\\', '/');
        type.Image = type.Image.Replace('\\', '/');
        return type;
    }
}

public class DiagramPointComponent : ModelComponent<DiagramPointParam, DiagramPointGoo, DiagramPoint>
{
    public override Guid ComponentGuid => new("61FB9BBE-64DE-42B2-B7EF-69CD97FDD9E3");
}

public class PieceComponent : ModelComponent<PieceParam, PieceGoo, Piece>
{
    public override Guid ComponentGuid => new("49CD29FC-F6EB-43D2-8C7D-E88F8520BA48");

    protected override void AddModelProps(dynamic pManager)
    {
        pManager.AddTextParameter("Id", "Id",
            "Id of the piece.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Type Name", "Na", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Variant", "Vn?",
            "The optional variant of the type of the piece. No variant means the default variant.",
            GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pn?",
            "The optional plane of the piece. When pieces are connected only one piece can have a plane.",
            GH_ParamAccess.item);
        pManager.AddParameter(new DiagramPointParam(), "Center", "Ce?",
            "The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
            GH_ParamAccess.item);
    }

    protected override void GetProps(IGH_DataAccess DA, dynamic pieceGoo)
    {
        var id = "";
        var typeName = "";
        var typeVariant = "";
        var plane = new Rhino.Geometry.Plane();
        var centerGoo = new DiagramPointGoo();

        if (DA.GetData(2, ref id))
            pieceGoo.Value.Id = id;
        if (DA.GetData(3, ref typeName))
            pieceGoo.Value.Type.Name = typeName;
        if (DA.GetData(4, ref typeVariant))
            pieceGoo.Value.Type.Variant = typeVariant;
        if (DA.GetData(5, ref plane))
            pieceGoo.Value.Plane = plane.Convert();
        if (DA.GetData(6, ref centerGoo))
            pieceGoo.Value.Center = centerGoo.Value;
    }

    protected override void SetData(IGH_DataAccess DA, dynamic pieceGoo)
    {
        DA.SetData(2, pieceGoo.Value.Id);
        DA.SetData(3, pieceGoo.Value.Type.Name);
        DA.SetData(4, pieceGoo.Value.Type.Variant);
        DA.SetData(5, (pieceGoo.Value.Plane as Plane)?.Convert());
        DA.SetData(6, pieceGoo.Value != null ? new DiagramPointGoo(pieceGoo.Value.Center as DiagramPoint) : null);
    }
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");

    protected override void AddModelProps(dynamic pManager)
    {
        pManager.AddTextParameter("Connected Piece Id", "CdPc", "Id of the connected piece.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Connected Piece Type Port Id", "CdPo?",
            "Optional id of the port of type of the piece. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Id", "CgPc", "Id of the connected piece.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Type Port Id", "CgPo?",
            "Optional id of the port of type of the piece. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?",
            "The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl?",
            "The optional horizontal tilt perpendicular to the port direction (applied after rotation) between the connected and the connecting piece in degrees.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Gap", "Gp?",
            "The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Shift", "Sf?",
            "The optional lateral shift (applied after rotation and tilt in port direction) between the connected and the connecting piece.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("X", "X?",
            "The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Y", "Y?",
            "The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
            GH_ParamAccess.item);
    }

    protected override void GetProps(IGH_DataAccess DA, dynamic connectionGoo)
    {
        var connectedPieceId = "";
        var connectedPortId = "";
        var connectingPieceId = "";
        var connectingPortId = "";
        var rotation = 0.0;
        var tilt = 0.0;
        var gap = 0.0;
        var shift = 0.0;
        var x = 0.0;
        var y = 0.0;

        if (DA.GetData(2, ref connectedPieceId))
            connectionGoo.Value.Connected.Piece.Id = connectedPieceId;
        if (DA.GetData(3, ref connectedPortId))
            connectionGoo.Value.Connected.Port.Id = connectedPortId;
        if (DA.GetData(4, ref connectingPieceId))
            connectionGoo.Value.Connecting.Piece.Id = connectingPieceId;
        if (DA.GetData(5, ref connectingPortId))
            connectionGoo.Value.Connecting.Port.Id = connectingPortId;
        if (DA.GetData(6, ref rotation))
            connectionGoo.Value.Rotation = (float)rotation;
        if (DA.GetData(7, ref tilt))
            connectionGoo.Value.Tilt = (float)tilt;
        if (DA.GetData(8, ref gap))
            connectionGoo.Value.Gap = (float)gap;
        if (DA.GetData(9, ref shift))
            connectionGoo.Value.Shift = (float)shift;
        if (DA.GetData(10, ref x))
            connectionGoo.Value.X = (float)x;
        if (DA.GetData(11, ref y))
            connectionGoo.Value.Y = (float)y;
    }

    protected override void SetData(IGH_DataAccess DA, dynamic connectionGoo)
    {
        DA.SetData(2, connectionGoo.Value.Connected.Piece.Id);
        DA.SetData(3, connectionGoo.Value.Connected.Port.Id);
        DA.SetData(4, connectionGoo.Value.Connecting.Piece.Id);
        DA.SetData(5, connectionGoo.Value.Connecting.Port.Id);
        DA.SetData(6, connectionGoo.Value.Rotation);
        DA.SetData(7, connectionGoo.Value.Tilt);
        DA.SetData(8, connectionGoo.Value.Gap);
        DA.SetData(9, connectionGoo.Value.Shift);
        DA.SetData(10, connectionGoo.Value.X);
        DA.SetData(11, connectionGoo.Value.Y);
    }
}

public class DesignComponent : ModelComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("AAD8D144-2EEE-48F1-A8A9-52977E86CB54");

    protected override Design ProcessModel(Design design)
    {
        if (design.Unit == "")
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                design.Unit = Utility.LengthUnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                design.Unit = "m";
            }

        design.Icon = design.Icon.Replace('\\', '/');
        design.Image = design.Image.Replace('\\', '/');

        return design;
    }
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");

    protected override Kit ProcessModel(Kit kit)
    {
        kit.Icon = kit.Icon.Replace('\\', '/');
        kit.Image = kit.Image.Replace('\\', '/');
        kit.Preview = kit.Preview.Replace('\\', '/');
        return kit;
    }
}

#endregion

public class RandomIdsComponent : Component
{
    public RandomIdsComponent()
        : base("Random Ids", "%Ids", "Generate random ids.", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("27E48D59-10BE-4239-8AAC-9031BF6AFBCC");

    protected override Bitmap Icon => Resources.id_random_24x24;

    public override GH_Exposure Exposure => GH_Exposure.secondary;

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
                var id = Semio.Utility.GenerateRandomId(BitConverter.ToInt32(hash, 0));
                ids.Add(id);
            }
        }

        DA.SetDataList(0, ids);
    }
}

#region Engine

public abstract class EngineComponent : Component
{
    protected EngineComponent(string name, string nickname, string description, string subcategory = "Persistence")
        : base(name, nickname, description, subcategory)
    {
    }

    protected virtual string RunDescription => "True to start the operation.";

    protected virtual string SuccessDescription => "True if the operation was successful.";

    protected virtual void RegisterEngineInputParams(GH_InputParamManager pManager)
    {
    }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterEngineInputParams(pManager);
        pManager.AddBooleanParameter("Run", "R", RunDescription, GH_ParamAccess.item, false);
    }

    protected virtual void RegisterEngineOutputParams(GH_OutputParamManager pManager)
    {
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", SuccessDescription, GH_ParamAccess.item);
        RegisterEngineOutputParams(pManager);
    }

    protected virtual dynamic? GetInput(IGH_DataAccess DA)
    {
        return null;
    }

    protected abstract dynamic? Run(dynamic? input = null);

    protected virtual void SetOutput(IGH_DataAccess DA, dynamic response)
    {
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var run = false;

        DA.GetData(Params.Input.Count - 1, ref run);
        if (!run) return;

        var input = GetInput(DA);
        try
        {
            var response = Run(input);
            SetOutput(DA, response);
            DA.SetData(0, true);
        }
        catch (ClientException e)
        {
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, e.Message);
            DA.SetData(0, false);
        }
        catch (ServerException e)
        {
            string serializedInput = input != null ? Semio.Utility.Serialize(input) : "";
            AddRuntimeMessage(GH_RuntimeMessageLevel.Error,
                "The engine didn't like it ¯\\_(ツ)_/¯\n" +
                "If you want, you can report this under: https://github.com/usalu/semio/issues\n" +
                $"Or write me an email: {Semio.Constants.Email}\n\n" +
                "ServerError: " + e.Message + "\n" +
                "Semio.Release: " + Semio.Constants.Release + "\n" +
                "Semio.Grasshopper: " + Constants.Version + "\n" +
                (serializedInput != ""
                    ? "Input: " + (serializedInput.Length < 1000
                        ? serializedInput
                        : serializedInput.Substring(0, 1000) + "\n...\n")
                    : ""));
            DA.SetData(0, false);
        }
    }

    protected override void BeforeSolveInstance()
    {
        base.BeforeSolveInstance();
        var processes = Process.GetProcessesByName("semio-engine");
        if (processes.Length == 0)
        {
            var path = Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? string.Empty,
                "semio-engine.exe");
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
            // TODO: Implement a status check and wait for the engine to be ready
            Thread.Sleep(1000);
        }
    }
}

#region Persistence

public abstract class PersistenceComponent : EngineComponent
{
    protected PersistenceComponent(string name, string nickname, string description, string subcategory = "Persistence")
        : base(name, nickname, description, subcategory)
    {
    }

    protected virtual void RegisterPersitenceInputParams(GH_InputParamManager pManager)
    {
    }

    protected override void RegisterEngineInputParams(GH_InputParamManager pManager)
    {
        RegisterPersitenceInputParams(pManager);
        var amountCustomParams = pManager.ParamCount;
        pManager.AddTextParameter("Uri", "Ur?",
            "Optional Unique Resource Identifier (URI) of the kit. This can be an absolute path to a local kit or a url to a remote kit.\n" +
            "If none is provided, it will try to see if the Grasshopper script is executed inside a local kit.",
            GH_ParamAccess.item);
        pManager[amountCustomParams].Optional = true;
    }

    protected virtual void RegisterPersitenceOutputParams(GH_OutputParamManager pManager)
    {
    }

    protected override void RegisterEngineOutputParams(GH_OutputParamManager pManager)
    {
        RegisterPersitenceOutputParams(pManager);
    }

    protected virtual dynamic? GetPersistentInput(IGH_DataAccess DA)
    {
        return null;
    }

    protected override dynamic? GetInput(IGH_DataAccess DA)
    {
        var uri = "";

        if (!DA.GetData(Params.Input.Count - 2, ref uri))
            uri = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        var input = GetPersistentInput(DA);

        return new { Uri = uri, Input = input };
    }

    protected abstract dynamic? RunOnKit(string url, dynamic? input);

    protected override dynamic? Run(dynamic? input = null)
    {
        var output = RunOnKit(input.Uri, input.Input);
        return output;
    }
}

public class LoadKitComponent : PersistenceComponent
{
    public LoadKitComponent() : base("Load Kit", "/Kit", "Load a kit.")
    {
    }

    protected override string RunDescription => "True to load the kit.";
    protected override string SuccessDescription => "True if the kit was successfully loaded. False otherwise.";

    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");

    protected override Bitmap Icon => Resources.kit_load_24x24;

    public override GH_Exposure Exposure => GH_Exposure.primary;

    protected override void RegisterPersitenceOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
        pManager.AddTextParameter("Local Directory", "Di",
            "The optional local directory of the kit. This applies to local kits and cached remote kits.",
            GH_ParamAccess.item);
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input)
    {
        var kit = Api.GetKit(uri);
        return kit;
    }

    protected override void SetOutput(IGH_DataAccess DA, dynamic response)
    {
        DA.SetData(1, new KitGoo(response));

        var uri = "";
        DA.GetData(0, ref uri);
        string directory;
        if (uri == "")
        {
            directory = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();
        }
        else if (uri.StartsWith("http") && uri.EndsWith(".zip"))
        {
            var encodedUri = Semio.Utility.Encode(uri);
            var userPath = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
            directory = Path.Combine(userPath, ".semio", "cache", encodedUri);
        }
        else
        {
            directory = uri;
        }

        DA.SetData(2, directory);
    }
}

public class CreateKitComponent : PersistenceComponent
{
    public CreateKitComponent() : base("Create Kit", "+Kit", "Create a kit.")
    {
    }

    protected override string RunDescription => "True to create the kit.";
    protected override string SuccessDescription => "True if the kit was successfully created. False otherwise.";

    public override Guid ComponentGuid => new("1CC1BE06-85B8-4B0E-A59A-35B4D7C6E0FD");

    protected override Bitmap Icon => Resources.kit_create_24x24;

    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterPersitenceInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
    }

    protected override dynamic GetPersistentInput(IGH_DataAccess DA)
    {
        var kitGoo = new KitGoo();
        DA.GetData(0, ref kitGoo);
        return kitGoo.Value;
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.CreateKit(uri, input);
        return null;
    }
}

public class DeleteKitComponent : PersistenceComponent
{
    public DeleteKitComponent() : base("Delete Kit", "-Kit", "Delete a kit.")
    {
    }

    protected override string RunDescription => "True to delete the kit.";
    protected override string SuccessDescription => "True if the kit was successfully deleted. False otherwise.";
    public override Guid ComponentGuid => new("38D4283C-510C-4E77-9105-92A5BE3E3BA0");

    protected override Bitmap Icon => Resources.kit_delete_24x24;

    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.DeleteKit(uri);
        return null;
    }
}

#region Putting

public abstract class PutComponent<T, U, V> : PersistenceComponent where T : ModelParam<U, V>, new()
    where U : ModelGoo<V>, new()
    where V : Model<V>, new()

{
    public static readonly string NameM;
    public static readonly ModelAttribute ModelM;

    static PutComponent()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMetaGrasshopper = Meta.Goo;

        NameM = typeof(V).Name;
        ModelM = Semio.Meta.Model[NameM];
    }

    protected PutComponent()
        : base($"Put {NameM}", $"+{ModelM.Abbreviation}",
            $"Put a {NameM.ToLower()} to the kit. If the same {NameM.ToLower()} (same name and variant) exists it will be overwritten")
    {
    }

    protected override string RunDescription => $"True to put the {NameM.ToLower()} to the kit.";
    protected override string SuccessDescription => $"True if the {NameM.ToLower()} was put to the kit.";

    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{NameM.ToLower()}_put_24x24");

    public override GH_Exposure Exposure => GH_Exposure.secondary;

    protected override void RegisterPersitenceInputParams(GH_InputParamManager pManager)
    {
        pManager.AddParameter(new T());
    }

    protected override dynamic GetPersistentInput(IGH_DataAccess DA)
    {
        var goo = new U();
        DA.GetData(0, ref goo);
        return goo.Value;
    }
}

public class PutTypeComponent : PutComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("BC46DC07-C0BE-433F-9E2F-60CCBAA39148");

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.PutType(uri, input);
        return null;
    }
}

public class PutDesignComponent : PutComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("8B7AA946-0CB1-4CA8-A712-610B60425368");

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.PutDesign(uri, input);
        return null;
    }
}

#endregion

#region Removing

public abstract class RemoveComponent<T, U, V> : PersistenceComponent where T : ModelParam<U, V>, new()
    where U : ModelGoo<V>, new()
    where V : Model<V>, new()
{
    public static readonly string NameM;
    public static readonly ModelAttribute ModelM;

    static RemoveComponent()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMetaGrasshopper = Meta.Goo;

        NameM = typeof(V).Name;
        ModelM = Semio.Meta.Model[NameM];
    }


    public RemoveComponent()
        : base($"Remove {NameM}", $"-{Semio.Meta.Model[NameM].Abbreviation}",
            $"Remove a {NameM.ToLower()} from a kit.")
    {
    }

    protected override string RunDescription => $"True to remove the {NameM.ToLower()} from the kit.";
    protected override string SuccessDescription => $"True if the {NameM.ToLower()} was removed from the kit.";

    protected override Bitmap Icon => (Bitmap)Resources.ResourceManager.GetObject($"{NameM.ToLower()}_remove_24x24");

    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterPersitenceInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter($"{NameM} Name", "Na", $"Name of the {NameM.ToLower()} to remove.",
            GH_ParamAccess.item);
        pManager.AddTextParameter($"{NameM} Variant", "Vn?",
            $"The optional variant of the {NameM.ToLower()} to remove. No variant means the default variant.",
            GH_ParamAccess.item);
        pManager[pManager.ParamCount - 1].Optional = true;
    }

    protected override dynamic GetPersistentInput(IGH_DataAccess DA)
    {
        var name = "";
        var variant = "";
        DA.GetData(0, ref name);
        DA.GetData(1, ref variant);
        return ConstructId(name, variant);
    }

    protected virtual dynamic ConstructId(string name, string variant)
    {
        return new { Name = name, Variant = variant };
    }
}

public class RemoveTypeComponent : RemoveComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("F38D0E82-5A58-425A-B705-7A62FD9DB957");

    protected override dynamic ConstructId(string name, string variant)
    {
        return new TypeId { Name = name, Variant = variant };
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.RemoveType(uri, input);
        return null;
    }
}

public class RemoveDesignComponent : RemoveComponent<DesignParam, DesignGoo, Design>
{
    public override Guid ComponentGuid => new("9ECCE095-9D1E-4554-A3EB-1EAEEE2B12D5");

    protected override dynamic ConstructId(string name, string variant)
    {
        return new DesignId { Name = name, Variant = variant };
    }

    protected override dynamic? RunOnKit(string uri, dynamic? input = null)
    {
        Api.RemoveDesign(uri, input);
        return null;
    }
}

#endregion

public class ClearCacheComponent : Component
{
    public ClearCacheComponent() : base("Clear Cache", "-Ca", "Clear the cache of all the remote kits.", "Persistence")
    {
    }

    public override Guid ComponentGuid => new("500BB2EA-56DE-4C38-9C5D-61B8EA0A8948");

    protected override Bitmap Icon => Resources.cache_clear_24x24;

    public override GH_Exposure Exposure => GH_Exposure.tertiary;

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddBooleanParameter("Run", "R", "True to clear the cache.", GH_ParamAccess.item, false);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddBooleanParameter("Success", "Sc", "True if the cache was successfully cleared.",
            GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var run = false;
        DA.GetData(0, ref run);
        DA.SetData(0, false);
        if (!run) return;

        try
        {
            // find process semio-engine.exe and kill it
            var processes = Process.GetProcessesByName("semio-engine");
            if (processes.Length > 0)
                foreach (var process in processes)
                {
                    process.Kill();
                    process.WaitForExit();
                }
        }
        catch (Exception e)
        {
        }

        var userPath = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        var cachePath = Path.Combine(userPath, ".semio", "cache");
        if (Directory.Exists(cachePath))
        {
            Directory.Delete(cachePath, true);
            Directory.CreateDirectory(cachePath);
        }

        DA.SetData(0, true);
    }
}

#endregion

#region Assistant

public abstract class AssistantComponent : EngineComponent
{
    public AssistantComponent(string name, string nickname, string description) : base(name, nickname, description,
        "Assistant")
    {
    }
}

public class PredictDesignComponent : AssistantComponent
{
    public PredictDesignComponent() : base("Predict Design", "%Dsn", "Predict a design.")
    {
    }

    protected override string RunDescription => "True to predict the design.";
    protected override string SuccessDescription => "True if the design was successfully predicted. False otherwise.";

    public override Guid ComponentGuid => new("1EAD6636-2D8C-47CC-894A-E4FE2465AAA7");

    protected override Bitmap Icon => Resources.design_predict_24x24;

    protected override void RegisterEngineInputParams(GH_InputParamManager pManager)
    {
        var pCount = pManager.ParamCount;
        pManager.AddTextParameter("Description", "Dc",
            "The description of the design or an instruction how to change the base design.", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "Ty+", "The types to use in the design.", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Design", "Dn?", "The optional design to use a base.",
            GH_ParamAccess.item);
        pManager[pCount + 2].Optional = true;
    }

    protected override void RegisterEngineOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new DesignParam(), "Design", "Dsn", "Predicted design.", GH_ParamAccess.item);
    }

    protected override dynamic? GetInput(IGH_DataAccess DA)
    {
        var description = "";
        var types = new List<TypeGoo>();
        var designGoo = new DesignGoo();

        DA.GetData(0, ref description);
        DA.GetDataList(1, types);
        Design? design;
        if (DA.GetData(2, ref designGoo))
            design = designGoo.Value;
        else
            design = null;

        return new { Description = description, Types = types.Select(t => t.Value).ToArray(), Design = design };
    }

    protected override dynamic? Run(dynamic? input = null)
    {
        var design = Api.PredictDesign(input.Description, input.Types, input.Design);
        return design;
    }

    protected override void SetOutput(IGH_DataAccess DA, dynamic response)
    {
        DA.SetData(1, new DesignGoo(response));
    }
}

#endregion

#endregion

#endregion

#region Meta

public static class Meta
{
    /// <summary>
    ///     Name of the model : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Goo;

    /// <summary>
    ///     Name of the model : Index of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyGoo;

    /// <summary>
    ///     Name of the model : Index of the property : Type
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyItemGoo;

    /// <summary>
    ///     Name of the model : Param
    /// </summary>
    public static readonly ImmutableDictionary<string, System.Type> Param;

    /// <summary>
    ///     Name of the model : Index of the property : Param
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<System.Type>> PropertyParam;

    /// <summary>
    ///     Name of the model : Index of the property : IsMapped
    /// </summary>
    public static readonly ImmutableDictionary<string, ImmutableArray<bool>> IsPropertyMapped;

    static Meta()
    {
        // force compiler to run static constructor of the the meta classes first.
        var dummyMeta = Semio.Meta.Model;

        var goo = new Dictionary<string, System.Type>();
        var propertyGoo = new Dictionary<string, List<System.Type>>();
        var propertyItemGoo = new Dictionary<string, List<System.Type>>();
        var param = new Dictionary<string, System.Type>();
        var propertyParam = new Dictionary<string, List<System.Type>>();
        var manualMappedTypes = new Dictionary<System.Type, (System.Type, System.Type)>
        {
            { typeof(string), (typeof(GH_String), typeof(Param_String)) },
            { typeof(bool), (typeof(GH_Boolean), typeof(Param_Boolean)) },
            { typeof(int), (typeof(GH_Integer), typeof(Param_Integer)) },
            { typeof(float), (typeof(GH_Number), typeof(Param_Number)) },
            { typeof(Point), (typeof(GH_Point), typeof(Param_Point)) },
            { typeof(Vector), (typeof(GH_Vector), typeof(Param_Vector)) },
            { typeof(Plane), (typeof(GH_Plane), typeof(Param_Plane)) }
        };
        var isPropertyMapped = new Dictionary<string, List<bool>>();
        foreach (var manualMappedTypeKvp in manualMappedTypes)
        {
            var name = manualMappedTypeKvp.Key.Name;
            goo[name] = manualMappedTypeKvp.Value.Item1;
            goo[name + "List"] = typeof(List<>).MakeGenericType(goo[name]);
            param[name] = manualMappedTypeKvp.Value.Item2;
        }

        foreach (var kvp in Semio.Meta.Type)
        {
            var baseName = typeof(Meta).Namespace + "." + kvp.Key;
            if (!goo.ContainsKey(kvp.Key))
            {
                var equivalentGooType = Assembly.GetExecutingAssembly().GetType(baseName + "Goo");
                if (equivalentGooType != null)
                {
                    goo[kvp.Key] = equivalentGooType;
                    goo[kvp.Key + "List"] = typeof(List<>).MakeGenericType(goo[kvp.Key]);
                }

                var equivalentParamType = Assembly.GetExecutingAssembly().GetType(baseName + "Param");
                if (equivalentParamType != null)
                    param[kvp.Key] = equivalentParamType;
            }

            propertyGoo[kvp.Key] = new List<System.Type>();
            propertyItemGoo[kvp.Key] = new List<System.Type>();
            propertyParam[kvp.Key] = new List<System.Type>();
            isPropertyMapped[kvp.Key] = new List<bool>();
        }

        foreach (var modelKvp in Semio.Meta.Property)
            for (var i = 0; i < modelKvp.Value.Length; i++)
            {
                var property = modelKvp.Value[i];
                var isPropertyList = Semio.Meta.IsPropertyList[modelKvp.Key][i];
                var propertyTypeName = isPropertyList
                    ? property.PropertyType.GetGenericArguments()[0].Name
                    : property.PropertyType.Name;
                bool isPropertyMappedValue;
                try
                {
                    isPropertyMappedValue = manualMappedTypes.ContainsKey(Semio.Meta.Type[propertyTypeName]);
                }
                catch
                {
                    isPropertyMappedValue = false;
                }

                isPropertyMapped[modelKvp.Key].Add(isPropertyMappedValue);
                try
                {
                    propertyGoo[modelKvp.Key].Add(goo[isPropertyList ? propertyTypeName + "List" : propertyTypeName]);
                    propertyItemGoo[modelKvp.Key].Add(goo[propertyTypeName]);
                }
                catch (Exception e)
                {
                    // ignored
                }

                try
                {
                    propertyParam[modelKvp.Key].Add(param[propertyTypeName]);
                }
                catch (Exception e)
                {
                    // ignored
                }
            }

        Goo = goo.ToImmutableDictionary();
        PropertyGoo = propertyGoo.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        PropertyItemGoo = propertyItemGoo.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        Param = param.ToImmutableDictionary();
        PropertyParam = propertyParam.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
        IsPropertyMapped = isPropertyMapped.ToImmutableDictionary(
            kvp => kvp.Key, kvp => kvp.Value.ToImmutableArray());
    }
}

#endregion