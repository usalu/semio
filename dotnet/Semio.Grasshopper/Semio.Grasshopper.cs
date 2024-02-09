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
using Rhino.Geometry;
using Semio.Grasshopper.Properties;

namespace Semio.Grasshopper;

// TODO: Add toplevel scanning for kits wherever a directory is given
// Maybe extension function for components. The repeated code looks something like this:
// if (!DA.GetData(_, ref path))
//      path = OnPingDocument().IsFilePathDefined
//          ? Path.GetDirectoryName(OnPingDocument().FilePath)
//          : Directory.GetCurrentDirectory();

#region Copilot

//public class Representation
//{
//    public string Url { get; set; }
//    public string Lod { get; set; }
//    public List<string> Tags { get; set; }
//}
//public class Specifier
//{
//    public string Context { get; set; }
//    public string Group { get; set; }
//}

//public class Point
//{
//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//}

//public class Vector
//{
//    public float X { get; set; }
//    public float Y { get; set; }
//    public float Z { get; set; }

//}

//public class Plane
//{
//    public Point Origin { get; set; }
//    public Vector XAxis { get; set; }
//    public Vector YAxis { get; set; }
//}

//public class Port
//{
//    public Plane Plane { get; set; }
//    public List<Specifier> Specifiers { get; set; }

//}

//public class PortId
//{
//    public List<Specifier> Specifiers { get; set; }
//}

//public class Quality
//{
//    public string Name { get; set; }
//    public string Value { get; set; }
//    public string Unit { get; set; }
//}

//public class Type
//{
//    public string Name { get; set; }
//    public string Explanation { get; set; }
//    public string Icon { get; set; }
//    public List<Representation> Representations { get; set; }
//    public List<Port> Ports { get; set; }
//    public List<Quality> Qualities { get; set; }

//}

//public class TypeId
//{
//    public string Name { get; set; }
//    public List<Quality> Qualities { get; set; }
//}

//public class Piece
//{
//    public string Id { get; set; }
//    public TypeId Type { get; set; }
//}

//public class TypePieceSide
//{
//    public PortId Port { get; set; }
//}

//public class PieceSide
//{
//    public string Id { get; set; }
//    public TypePieceSide Type { get; set; }
//}

//public class Side
//{
//    public PieceSide Piece { get; set; }
//}

//public class Attraction
//{
//    public Side Attracting { get; set; }
//    public Side Attracted { get; set; }
//}

//public class Formation
//{
//    public string Name { get; set; }
//    public string Explanation { get; set; }
//    public string Icon { get; set; }
//    public List<Piece> Pieces { get; set; }
//    public List<Attraction> Attractions { get; set; }
//    public List<Quality> Qualities { get; set; }
//}

//public class Kit
//{
//    public string Name { get; set; }
//    public string Explanation { get; set; }
//    public string Icon { get; set; }
//    public string Url { get; set; }
//    public List<Type> Types { get; set; }
//    public List<Formation> Formations { get; set; }
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
        pManager.AddTextParameter("Level of Detail", "Ld?", "Optional level of detail of the representation.",
            GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Tags", "Tg*", "Optional tags for the representation.", GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new RepresentationParam(), "Representation", "Rp",
            "Constructed or modified representation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Url", "Ur", "Url of the representation. Either a relative file path or link.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Level of Detail", "Ld?", "Optional level of detail of the representation.",
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
        DA.GetData(1, ref url);
        DA.GetData(2, ref lod);
        DA.GetDataList(3, tags);

        if (url != "") representationGoo.Value.Url = url;

        if (lod != "") representationGoo.Value.Lod = lod;

        if (tags.Count > 0) representationGoo.Value.Tags = tags;

        DA.SetData(0, representationGoo.Duplicate());
        DA.SetData(1, representationGoo.Value.Url);
        if (representationGoo.Value.Lod != null)
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
        DA.GetData(1, ref context);
        DA.GetData(2, ref group);

        if (context != "") specifierGoo.Value.Context = context;

        if (group != "") specifierGoo.Value.Group = group;

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
        var planeInput = false;
        var planeGeo = new Rhino.Geometry.Plane();
        var specifierGoos = new List<SpecifierGoo>();

        DA.GetData(0, ref portGoo);
        if (DA.GetData(1, ref planeGeo))
            portGoo.Value.Plane = planeGeo.convert();
        DA.GetDataList(2, specifierGoos);

        if (specifierGoos.Count > 0) portGoo.Value.Specifiers = specifierGoos.Select(s => s.Value).ToList();

        DA.SetData(0, portGoo.Duplicate());
        DA.SetData(1, portGoo.Value.Plane?.convert());
        if (portGoo.Value.Specifiers != null)
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
        DA.GetData(1, ref name);
        DA.GetData(2, ref value);
        DA.GetData(3, ref unit);

        if (name != "") qualityGoo.Value.Name = name;

        if (value != "") qualityGoo.Value.Value = value;

        if (unit != "") qualityGoo.Value.Unit = unit;

        DA.SetData(0, qualityGoo.Duplicate());
        DA.SetData(1, qualityGoo.Value.Name);
        DA.SetData(2, qualityGoo.Value.Value);
        if (qualityGoo.Value.Unit != null)
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
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the type.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the type.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+", "Representations of the type.",
            GH_ParamAccess.list);
        pManager[4].Optional = true;
        pManager.AddParameter(new PortParam(), "Ports", "Po+", "Ports of the type.", GH_ParamAccess.list);
        pManager[5].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the type. They can be further used to distinguish types with the same name.",
            GH_ParamAccess.list);
        pManager[6].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new TypeParam(), "Type", "Ty",
            "Constructed or modified type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the type.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the type.", GH_ParamAccess.item);
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
        var explanation = "";
        var icon = "";
        var representationGoos = new List<RepresentationGoo>();
        var portGoos = new List<PortGoo>();
        var qualityGoos = new List<QualityGoo>();

        DA.GetData(0, ref typeGoo);
        DA.GetData(1, ref name);
        DA.GetData(2, ref explanation);
        DA.GetData(3, ref icon);
        DA.GetDataList(4, representationGoos);
        DA.GetDataList(5, portGoos);
        DA.GetDataList(6, qualityGoos);

        if (name != "") typeGoo.Value.Name = name;

        if (explanation != "") typeGoo.Value.Explanation = explanation;

        if (icon != "") typeGoo.Value.Icon = icon;

        if (representationGoos.Count > 0)
            typeGoo.Value.Representations = representationGoos.Select(r => r.Value).ToList();

        if (portGoos.Count > 0) typeGoo.Value.Ports = portGoos.Select(p => p.Value).ToList();

        if (qualityGoos.Count > 0) typeGoo.Value.Qualities = qualityGoos.Select(q => q.Value).ToList();

        DA.SetData(0, typeGoo.Duplicate());
        DA.SetData(1, typeGoo.Value.Name);
        if (typeGoo.Value.Explanation != null)
            DA.SetData(2, typeGoo.Value.Explanation);
        if (typeGoo.Value.Icon != null)
            DA.SetData(3, typeGoo.Value.Icon);
        if (typeGoo.Value.Representations != null)
            DA.SetDataList(4, typeGoo.Value.Representations.Select(r => new RepresentationGoo(r.DeepClone())));
        if (typeGoo.Value.Ports != null)
            DA.SetDataList(5, typeGoo.Value.Ports.Select(p => new PortGoo(p.DeepClone())));
        if (typeGoo.Value.Qualities != null)
            DA.SetDataList(6, typeGoo.Value.Qualities.Select(q => new QualityGoo(q.DeepClone())));
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
        pManager.AddTextParameter("TypeName", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddParameter(new QualityParam(), "TypeQualities", "TyQl*",
            "If there is more than one type with the same name use qualities to precisely identify the type.",
            GH_ParamAccess.list);
        pManager[3].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new PieceParam(), "Piece", "Pc",
            "Constructed or modified piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Id", "Id", "Id of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("TypeName", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager.AddParameter(new QualityParam(), "TypeQualities", "TyQl*",
            "Optional qualities of the type of the piece.", GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var pieceGoo = new PieceGoo();
        var id = "";
        var typeName = "";
        var typeQualityGoos = new List<QualityGoo>();

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

        if (DA.GetDataList(3, typeQualityGoos))
            pieceGoo.Value.Type.Qualities = typeQualityGoos.Select(q => q.Value).ToList();

        DA.SetData(0, pieceGoo.Duplicate());
        DA.SetData(1, pieceGoo.Value.Id);
        DA.SetData(2, pieceGoo.Value.Type?.Name);
        if (pieceGoo.Value.Type?.Qualities != null)
            DA.SetDataList(3, pieceGoo.Value.Type.Qualities.Select(q => new QualityGoo(q.DeepClone())));
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
        var specifierGoos = new List<SpecifierGoo>();

        DA.GetData(0, ref sideGoo);
        DA.GetData(1, ref pieceId);
        DA.GetDataList(2, specifierGoos);

        if (pieceId != "")
        {
            if (sideGoo.Value.Piece == null)
                sideGoo.Value.Piece = new PieceSide { Id = pieceId };
            else
                sideGoo.Value.Piece.Id = pieceId;
        }

        if (specifierGoos.Count > 0)
        {
            if (sideGoo.Value.Piece == null)
                sideGoo.Value.Piece = new PieceSide
                {
                    Type = new TypePieceSide
                        { Port = new PortId { Specifiers = specifierGoos.Select(s => s.Value).ToList() } }
                };
            else if (sideGoo.Value.Piece.Type == null)
                sideGoo.Value.Piece.Type = new TypePieceSide
                    { Port = new PortId { Specifiers = specifierGoos.Select(s => s.Value).ToList() } };
            else if (sideGoo.Value.Piece.Type.Port == null)
                sideGoo.Value.Piece.Type.Port = new PortId { Specifiers = specifierGoos.Select(s => s.Value).ToList() };
            else
                sideGoo.Value.Piece.Type.Port.Specifiers = specifierGoos.Select(s => s.Value).ToList();
        }

        DA.SetData(0, sideGoo.Duplicate());
        if (sideGoo.Value.Piece != null)
            DA.SetData(1, sideGoo.Value.Piece.Id);
        if (sideGoo.Value.Piece?.Type?.Port?.Specifiers != null)
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
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the formation.", GH_ParamAccess.item);
        pManager[2].Optional = true;
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the formation.", GH_ParamAccess.item);
        pManager[3].Optional = true;
        pManager.AddParameter(new PieceParam(), "Pieces", "Pc+", "Pieces of the formation.", GH_ParamAccess.list);
        pManager[4].Optional = true;
        pManager.AddParameter(new AttractionParam(), "Attractions", "At+", "Attractions of the formation.",
            GH_ParamAccess.list);
        pManager[5].Optional = true;
        pManager.AddParameter(new QualityParam(), "Qualities", "Ql*",
            "Optional qualities of the formation. They can be further used to distinguish formations with the same name.",
            GH_ParamAccess.list);
        pManager[6].Optional = true;
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new FormationParam(), "Formation", "Fo",
            "Constructed or modified formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Name", "Na", "Name of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the formation.", GH_ParamAccess.item);
        pManager.AddTextParameter("Icon", "Ic?", "Optional icon of the formation.", GH_ParamAccess.item);
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
        var explanation = "";
        var icon = "";
        var pieceGoos = new List<PieceGoo>();
        var attractionGoos = new List<AttractionGoo>();
        var qualityGoos = new List<QualityGoo>();

        DA.GetData(0, ref formationGoo);
        DA.GetData(1, ref name);
        DA.GetData(2, ref explanation);
        DA.GetData(3, ref icon);
        DA.GetDataList(4, pieceGoos);
        DA.GetDataList(5, attractionGoos);
        DA.GetDataList(6, qualityGoos);

        if (name != "") formationGoo.Value.Name = name;

        if (explanation != "") formationGoo.Value.Explanation = explanation;

        if (icon != "") formationGoo.Value.Icon = icon;

        if (pieceGoos.Count > 0) formationGoo.Value.Pieces = pieceGoos.Select(p => p.Value).ToList();

        if (attractionGoos.Count > 0) formationGoo.Value.Attractions = attractionGoos.Select(a => a.Value).ToList();

        if (qualityGoos.Count > 0) formationGoo.Value.Qualities = qualityGoos.Select(q => q.Value).ToList();

        DA.SetData(0, formationGoo.Duplicate());
        DA.SetData(1, formationGoo.Value.Name);
        if (formationGoo.Value.Explanation != null)
            DA.SetData(2, formationGoo.Value.Explanation);
        if (formationGoo.Value.Icon != null)
            DA.SetData(3, formationGoo.Value.Icon);
        if (formationGoo.Value.Pieces != null)
            DA.SetDataList(4, formationGoo.Value.Pieces.Select(p => new PieceGoo(p.DeepClone())));
        if (formationGoo.Value.Attractions != null)
            DA.SetDataList(5, formationGoo.Value.Attractions.Select(a => new AttractionGoo(a.DeepClone())));
        if (formationGoo.Value.Qualities != null)
            DA.SetDataList(6, formationGoo.Value.Qualities.Select(q => new QualityGoo(q.DeepClone())));
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
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the kit", GH_ParamAccess.item);
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
            DA.SetData(1, kit.Explanation);
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
        pManager.AddTextParameter("Explanation", "Ex?", "Optional explanation of the kit", GH_ParamAccess.item);
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
        var explanation = "";
        var icon = "";
        var url = "";
        var typeGoos = new List<TypeGoo>();
        var formationGoos = new List<FormationGoo>();
        var path = "";
        var run = false;

        DA.GetData(0, ref name);
        DA.GetData(1, ref explanation);
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
            Explanation = explanation,
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
        if (typeQualityGoos.Count > 0) type.Qualities = typeQualityGoos.Select(q => q.Value).ToList();
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
        if (formationQualityGoos.Count > 0) formation.Qualities = formationQualityGoos.Select(q => q.Value).ToList();
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

        var response = new Api().FormationToSceneFromLocalKit(path, new FormationId
        {
            Name = formationName,
            Qualities = formationQualityGoos.Select(q => q.Value).ToList()
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