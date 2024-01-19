using System;
using System.Collections.Generic;
using System.Linq;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;

namespace Semio.Grasshopper;

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

#region Converters

public static class RhinoConverter
{
    public static Rhino.Geometry.Plane convert(this Plane plane)
    {
        return new Rhino.Geometry.Plane(
                       new Rhino.Geometry.Point3d(plane.Origin.X, plane.Origin.Y, plane.Origin.Z),
                                  new Rhino.Geometry.Vector3d(plane.XAxis.X, plane.XAxis.Y, plane.XAxis.Z),
                                  new Rhino.Geometry.Vector3d(plane.YAxis.X, plane.YAxis.Y, plane.YAxis.Z)
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
        return new RepresentationGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new SpecifierGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new PortGoo(Value);
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
                               new Rhino.Geometry.Point3d(Value.Plane.Origin.X, Value.Plane.Origin.Y, Value.Plane.Origin.Z),
                                              new Rhino.Geometry.Vector3d(Value.Plane.XAxis.X, Value.Plane.XAxis.Y, Value.Plane.XAxis.Z),
                                              new Rhino.Geometry.Vector3d(Value.Plane.YAxis.X, Value.Plane.YAxis.Y, Value.Plane.YAxis.Z)
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
        return new QualityGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new TypeGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new PieceGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new SideGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new AttractionGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new FormationGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
        return new KitGoo(Value);
    }

    public override string ToString()
    {
        return Value.ToString();
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
    public QualityParam() : base("Quality", "Qa", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("F2F6F2F9-7F0E-4F0F-9F0C-7F6F6F6F6F6F");

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
    public TypeParam() : base("Type", "T", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");

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
    public AttractionParam() : base("Attraction", "A", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("8B78CE81-27D6-4A07-9BF3-D862796B2FA4");

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
    public FormationParam() : base("Formation", "F", "", "semio", "Params")
    {
    }

    public override Guid ComponentGuid => new("1FB90496-93F2-43DE-A558-A7D6A9FE3596");

    protected override GH_GetterResult Prompt_Singular(ref FormationGoo value)
    {
        throw new NotImplementedException();
    }

    protected override GH_GetterResult Prompt_Plural(ref List<FormationGoo> values)
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

public class LoadKitComponent : SemioComponent
{
    public LoadKitComponent()
        : base("LoadKit", "/Kit",
            "",
            "semio", "Loading/Saving")
    {
    }

    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Directory", "D?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
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

        DA.GetData(0, ref path);
        DA.GetData(1, ref run);

        if (run)
        {
            var kit = new Api().LoadLocalKit(path);

            DA.SetData(0, kit.Name);
            DA.SetData(1, kit.Explanation);
            DA.SetData(2, kit.Icon);
            DA.SetData(3, kit.Url);
            DA.SetDataList(4, kit.Types.Select(t => new TypeGoo(t)));
            DA.SetDataList(5, kit.Formations.Select(f => new FormationGoo(f)));
        }
    }
}

public class RepresentationComponent : SemioComponent
{
    public RepresentationComponent()
        : base("~Representation", "~Rep",
            "Construct, deconstruct or modify a representation.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("37228B2F-70DF-44B7-A3B6-781D5AFCE122");

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

        DA.SetData(0, representationGoo);
        DA.SetData(1, representationGoo.Value.Url);
        DA.SetData(2, representationGoo.Value.Lod);
        DA.SetDataList(3, representationGoo.Value.Tags);
    }
}

public class SpecifierComponent : SemioComponent
{
    public SpecifierComponent()
        : base("~Specifier", "~Spc",
            "Construct, deconstruct or modify a specifier.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("2552DB71-8459-4DB5-AD66-723573E771A2");

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

        DA.SetData(0, specifierGoo);
        DA.SetData(1, specifierGoo.Value.Context);
        DA.SetData(2, specifierGoo.Value.Group);
    }
}

public class PortComponent : SemioComponent
{
    public PortComponent()
        : base("~Port", "~Por",
            "Construct, deconstruct or modify a port.",
            "semio", "Modelling")
    {
    }

    public override Guid ComponentGuid => new("E505C90C-71F4-413F-82FE-65559D9FFAB5");

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
        pManager.AddParameter(new SpecifierParam(), "Specifiers", "Sp+", "Specifiers of the port.",
            GH_ParamAccess.list);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var portGoo = new PortGoo();
        var planeInput = false;
        var planeGeo = new Rhino.Geometry.Plane();
        var specifierGoos = new List<SpecifierGoo>();

        DA.GetData(0, ref portGoo);
        if (DA.GetData(1, ref planeGeo)) planeInput = true;
        DA.GetDataList(2, specifierGoos);

        if (planeInput)
            portGoo.Value.Plane = RhinoConverter.convert(planeGeo);

        if (specifierGoos.Count > 0) portGoo.Value.Specifiers = specifierGoos.Select(s => s.Value).ToList();

        DA.SetData(0, portGoo);
        DA.SetData(1, portGoo.Value.Plane!=null ? RhinoConverter.convert(portGoo.Value.Plane) : null );
        DA.SetDataList(2, portGoo.Value.Specifiers.Select(s => new SpecifierGoo(s)));
    }
}

#endregion