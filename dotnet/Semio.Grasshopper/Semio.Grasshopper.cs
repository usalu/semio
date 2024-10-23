//Semio.Grasshopper.cs
//Copyright (C) 2024 Ueli Saluz

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

using System.Collections.Immutable;
using System.Diagnostics;
using System.Drawing;
using System.Reflection;
using System.Text;
using GH_IO.Serialization;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Parameters;
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
    public static object Convert(this object value) => value;
    public static string Convert(this string value) => value;
    public static int Convert(this int value) => value;
    public static float Convert(this double value) => (float)value;

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

    public override string TypeName => Value.GetType().Name;

    public override string TypeDescription =>
        ((ModelAttribute)Attribute.GetCustomAttribute(typeof(T), typeof(ModelAttribute))).Description;

    public override IGH_Goo Duplicate()
    {
        var duplicate = (ModelGoo<T>)Activator.CreateInstance(GetType());
        duplicate.Value = (T)Value.DeepClone();
        return duplicate;
    }

    public override string ToString()
    {
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

public class TypeGoo : ModelGoo<Type>
{
    public TypeGoo()
    {
    }

    public TypeGoo(Type value) : base(value)
    {
    }
}

public class ScreenPointGoo : ModelGoo<ScreenPoint>
{
    public ScreenPointGoo()
    {
    }

    public ScreenPointGoo(ScreenPoint value) : base(value)
    {
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

public class PieceGoo : ModelGoo<Piece>
{
    public PieceGoo()
    {
    }

    public PieceGoo(Piece value) : base(value)
    {
    }
}

public class SideGoo : ModelGoo<Side>
{
    public SideGoo()
    {
    }

    public SideGoo(Side value) : base(value)
    {
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
        ((ModelAttribute)Attribute.GetCustomAttribute(typeof(U), typeof(ModelAttribute))).Description, "semio",
        "Params")
    {
    }

    protected override Bitmap Icon
    {
        get
        {
            var iconName = $"{typeof(U).Name.ToLower()}_24x24";
            return (Bitmap)Resources.ResourceManager.GetObject(iconName);
        }
    }

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

public class TypeParam : ModelParam<TypeGoo, Type>
{
    public override Guid ComponentGuid => new("301FCFFA-2160-4ACA-994F-E067C4673D45");
}

public class ScreenPointParam : ModelParam<ScreenPointGoo, ScreenPoint>
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
        name, nickname, description, "semio", subcategory)
    {
    }
}

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
        $"Construct, deconstruct or modify a {NameM.ToLower()}", "Modelling")
    {
    }

    protected override Bitmap Icon
    {
        get
        {
            var iconName = $"{typeof(V).Name.ToLower()}_modify_24x24";
            return (Bitmap)Resources.ResourceManager.GetObject(iconName);
        }
    }

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
        // Input
        dynamic modelGoo = Activator.CreateInstance(GooM);
        if (DA.GetData(0, ref modelGoo))
            modelGoo = modelGoo.Duplicate();
        GetProps(DA, modelGoo);

        // Process
        modelGoo.Value = ProcessModel(modelGoo.Value);
        var (isValid, errors) = ((bool, List<string>))modelGoo.Value.Validate();
        foreach (var error in errors)
            AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, error);

        // Output
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
            bool hasInput = isList ? DA.GetDataList(i + 1, value) : DA.GetData(i + 1, ref value);
            if (hasInput)
            {
                if (isList)
                {
                    var listType = typeof(List<>).MakeGenericType(itemType);
                    dynamic list = Activator.CreateInstance(listType);
                    foreach (var item in gooValue)
                        list.Add((itemType == typeof(string) || itemType == typeof(int) || itemType == typeof(float)) ? item.Value : item.Value.DeepClone());
                
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
                DA.SetDataList(i + 1, value);
            else
                DA.SetData(i + 1, value);
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

public class TypeComponent : ModelComponent<TypeParam, TypeGoo, Type>
{
    public override Guid ComponentGuid => new("7E250257-FA4B-4B0D-B519-B0AD778A66A7");

    protected override Type ProcessModel(Type type)
    {
        if (type.Unit == "")
            try
            {
                var documentUnits = RhinoDoc.ActiveDoc.ModelUnitSystem;
                type.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                type.Unit = "m";
            }

        return type;
    }
}

public class ScreenPointComponent : ModelComponent<ScreenPointParam, ScreenPointGoo, ScreenPoint>
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
        pManager.AddTextParameter("Type Name", "TyNa", "Name of the type of the piece.", GH_ParamAccess.item);
        pManager.AddTextParameter("Type Variant", "TyVn?",
            "The optional variant of the type of the piece. No variant means the default variant.",
            GH_ParamAccess.item);
        pManager.AddPlaneParameter("Plane", "Pn?",
            "The optional plane of the piece. When pieces are connected only one piece can have a plane.",
            GH_ParamAccess.item);
        pManager.AddParameter(new ScreenPointParam(), "Screen Point", "SP?",
            "The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.",
            GH_ParamAccess.item);
    }

    protected override void GetProps(IGH_DataAccess DA, dynamic pieceGoo)
    {
        var id = "";
        var typeName = "";
        var typeVariant = "";
        var rootPlane = new Rhino.Geometry.Plane();
        var screenPointGoo = new ScreenPointGoo();

        if (DA.GetData(1, ref id))
            pieceGoo.Value.Id = id;
        if (DA.GetData(2, ref typeName))
            pieceGoo.Value.Type.Name = typeName;
        if (DA.GetData(3, ref typeVariant))
            pieceGoo.Value.Type.Variant = typeVariant;
        if (DA.GetData(4, ref rootPlane))
            pieceGoo.Value.Plane = rootPlane.Convert();
        if (DA.GetData(5, ref screenPointGoo))
            pieceGoo.Value.ScreenPoint = screenPointGoo.Value;
    }

    protected override void SetData(IGH_DataAccess DA, dynamic pieceGoo)
    {
        DA.SetData(1, pieceGoo.Value.Id);
        DA.SetData(2, pieceGoo.Value.Type.Name);
        DA.SetData(3, pieceGoo.Value.Type.Variant);
        DA.SetData(4, (pieceGoo.Value.Plane as Plane)?.Convert());
        DA.SetData(5, new ScreenPointGoo(pieceGoo.Value.ScreenPoint as ScreenPoint));
    }
}

public class ConnectionComponent : ModelComponent<ConnectionParam, ConnectionGoo, Connection>
{
    public override Guid ComponentGuid => new("AB212F90-124C-4985-B3EE-1C13D7827560");
    protected override void AddModelProps(dynamic pManager)
    {
        pManager.AddTextParameter("Connected Piece Id", "CdPc", "Id of the connected piece of the side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connected Piece Type Port Id", "CdPo?",
            "Optional id of the port of type of the piece of the side. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Id", "CnPc", "Id of the connected piece of the side.", GH_ParamAccess.item);
        pManager.AddTextParameter("Connecting Piece Type Port Id", "CnPo?",
            "Optional id of the port of type of the piece of the side. Otherwise the default port will be selected.",
            GH_ParamAccess.item);
        pManager.AddNumberParameter("Rotation", "Rt?", "The optional rotation between the connected and the connecting piece in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Tilt", "Tl?", "The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.", GH_ParamAccess.item);
        pManager.AddNumberParameter("Offset", "Of?", "The optional offset distance (in port direction after rotation and tilt) between the connected and the connecting piece.", GH_ParamAccess.item);
    }

    protected override void GetProps(IGH_DataAccess DA, dynamic connectionGoo)
    {
        var connectedPieceId = "";
        var connectedPieceTypePortId = "";
        var connectingPieceId = "";
        var connectingPieceTypePortId = "";
        var rotation = 0.0;
        var tilt = 0.0;
        var offset = 0.0;

        if (DA.GetData(1, ref connectedPieceId))
            connectionGoo.Value.Connected.Piece.Id = connectedPieceId;
        if (DA.GetData(2, ref connectedPieceTypePortId))
            connectionGoo.Value.Connected.Piece.Type.Port.Id = connectedPieceTypePortId;
        if (DA.GetData(3, ref connectingPieceId))
            connectionGoo.Value.Connecting.Piece.Id = connectingPieceId;
        if (DA.GetData(4, ref connectingPieceTypePortId))
            connectionGoo.Value.Connecting.Piece.Type.Port.Id = connectingPieceTypePortId;
        if (DA.GetData(5, ref rotation))
            connectionGoo.Value.Rotation = (float)rotation;
        if (DA.GetData(6, ref tilt))
            connectionGoo.Value.Tilt = (float)tilt;
        if (DA.GetData(7, ref offset))
            connectionGoo.Value.Offset = (float)offset;
    }

    protected override void SetData(IGH_DataAccess DA, dynamic connectionGoo)
    {
        DA.SetData(1, connectionGoo.Value.Connected.Piece.Id);
        DA.SetData(2, connectionGoo.Value.Connected.Piece.Type.Port.Id);
        DA.SetData(3, connectionGoo.Value.Connecting.Piece.Id);
        DA.SetData(4, connectionGoo.Value.Connecting.Piece.Type.Port.Id);
        DA.SetData(5, connectionGoo.Value.Rotation);
        DA.SetData(6, connectionGoo.Value.Tilt);
        DA.SetData(7, connectionGoo.Value.Offset);
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
                design.Unit = Utility.UnitSystemToAbbreviation(documentUnits);
            }
            catch (Exception e)
            {
                design.Unit = "m";
            }

        return design;
    }
}

public class KitComponent : ModelComponent<KitParam, KitGoo, Kit>
{
    public override Guid ComponentGuid => new("987560A8-10D4-43F6-BEBE-D71DC2FD86AF");
}

//public class RandomIdsComponent : Component
//{
//    public RandomIdsComponent()
//        : base("Random Ids", "%Ids",
//            "Generate random ids.",
//            "semio", "Modelling")
//    {
//    }

//    public override Guid ComponentGuid => new("27E48D59-10BE-4239-8AAC-9031BF6AFBCC");

//    protected override Bitmap Icon => Resources.id_random_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddIntegerParameter("Count", "Ct", "Number of ids to generate.", GH_ParamAccess.item, 1);
//        pManager.AddIntegerParameter("Seed", "Se", "Seed for the random generator.", GH_ParamAccess.item, 0);
//        pManager.AddBooleanParameter("Unique Component", "UC",
//            "If true, the generated ids will be unique for this component.", GH_ParamAccess.item, true);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Ids", "Id+", "Generated ids.", GH_ParamAccess.list);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var count = 0;
//        var seed = 0;
//        var unique = true;

//        DA.GetData(0, ref count);
//        DA.GetData(1, ref seed);
//        DA.GetData(2, ref unique);

//        var ids = new List<string>();

//        for (var i = 0; i < count; i++)
//        {
//            var hashString = seed + ";" + i;
//            if (unique)
//                hashString += ";" + InstanceGuid;
//            using (var md5 = MD5.Create())
//            {
//                var hash = md5.ComputeHash(Encoding.UTF8.GetBytes(hashString));
//                var id = Generator.GenerateRandomId(BitConverter.ToInt32(hash, 0));
//                ids.Add(id);
//            }
//        }

//        DA.SetDataList(0, ids);
//    }
//}

#endregion

#region Loading/Saving

public abstract class EngineComponent : Component
{
    protected static string SuccessDescription = "True if the operation was successful.";

    protected EngineComponent(string name, string nickname, string description)
        : base(name, nickname, description, "Loading/Saving")
    {
    }

    protected virtual void RegisterCustomInputParams(GH_InputParamManager pManager)
    {
    }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        RegisterCustomInputParams(pManager);
        int amountCustomParams = pManager.ParamCount;
        pManager.AddTextParameter("Directory", "Di?",
            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
            GH_ParamAccess.item);
        pManager[amountCustomParams].Optional = true;
        pManager.AddBooleanParameter("Run", "R", "Add the type to the kit.", GH_ParamAccess.item, false);
    }

    protected virtual void RegisterCustomOutputParams(GH_OutputParamManager pManager)
    {
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        RegisterCustomOutputParams(pManager);
        pManager.AddBooleanParameter("Success", "Sc", SuccessDescription, GH_ParamAccess.item);
    }

    protected abstract dynamic Run(string url);

    protected virtual void SetOutput(IGH_DataAccess DA, dynamic response)
    {
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var url = "";
        var run = false;

        if (!DA.GetData(0, ref url))
            url = OnPingDocument().IsFilePathDefined
                ? Path.GetDirectoryName(OnPingDocument().FilePath)
                : Directory.GetCurrentDirectory();

        DA.GetData(1, ref run);
        if (!run) return;

        var response = Run(url);

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

        SetOutput(DA, response);
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
        }
    }
}

public class LoadKitComponent : EngineComponent
{
    protected new static string SuccessDescription = "True if the kit was successfully loaded. False otherwise.";
    public LoadKitComponent() : base("Load Kit", "/Kit", "Load a kit.")
    {
    }

    public override Guid ComponentGuid => new("5BE3A651-581E-4595-8DAC-132F10BD87FC");

    protected override Bitmap Icon => Resources.kit_load_24x24;

    protected override void RegisterCustomOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam());
    }

    protected override dynamic Run(string url)
    {
        //return new Api().LoadLocalKit(url);
        return null;
    }

    protected override void SetOutput(IGH_DataAccess DA, dynamic response)
    {
        DA.SetData(0, new KitGoo(response.Value));
    }
}

//public class CreateKitComponent : EngineComponent
//{
//    protected new static string SuccessDescription = "True if the kit was successfully created. False otherwise.";

//    public CreateKitComponent() : base("Create Kit", "+Kit", "Create a kit.")
//    {
//    }

//    public override Guid ComponentGuid => new("1CC1BE06-85B8-4B0E-A59A-35B4D7C6E0FD");

//    protected override Bitmap Icon => Resources.kit_create_24x24;

//    protected override void RegisterCustomInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new KitParam());
//    }

//    protected override void Run(string path, IGH_DataAccess DA)
//    {
//        var kitGoo = new KitGoo();

//        var response = new Api().CreateLocalKit(path, kitGoo.Value);
//        if (response == null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
//            DA.SetData(0, false);
//            return;
//        }

//        if (response.Error != null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
//            DA.SetData(0, false);
//            return;
//        }

//        DA.SetData(0, true);
//    }
//}

//public class DeleteKitComponent : EngineComponent
//{
//    public DeleteKitComponent() : base("Delete Kit", "-Kit", "Delete a kit.")
//    {
//    }

//    public override Guid ComponentGuid => new("38D4283C-510C-4E77-9105-92A5BE3E3BA0");

//    protected override Bitmap Icon => Resources.kit_delete_24x24;

//    protected override dynamic Run(string url)=>new Api().DeleteLocalKit(path);

//}

//#region Adding

//public abstract class AddComponent<T> : EngineComponent where T : Model<T>
//{
//    protected new static string SuccessDescription = $"True if the {typeof(T).Name} was added to the kit.";

//    public AddComponent()
//        : base($"Add {typeof(T).Name}", $"+{Semio.Meta.Model[typeof(T).Name].Abbreviation}",
//            $"Add a {typeof(T).Name} to a kit.")
//    {
//    }

//    protected override Bitmap Icon
//    {
//        get
//        {
//            var iconName = $"{typeof(T).Name.ToLower()}_add_24x24";
//            return (Bitmap)Resources.ResourceManager.GetObject(iconName);
//        }
//    }

//    protected override void RegisterCustomInputParams(GH_InputParamManager pManager)
//    {
//        var param = (IGH_Param)Activator.CreateInstance(Meta.Param[typeof(T).Name]);
//        pManager.AddParameter(param, $"{typeof(T).Name}", $"{Semio.Meta.Model[typeof(T).Name].Code}",
//            $"{typeof(T).Name} to add to the kit.", GH_ParamAccess.item);
//    }

//    protected override void Run(string url, IGH_DataAccess DA)
//    {
//        var goo = Activator.CreateInstance(Meta.Goo[typeof(T).Name]);

//        if (DA.GetData(0, ref goo))
//            goo = goo.Duplicate();
//    }
//}

//public class AddTypeComponent : AddComponent<Type>
//{
//    public override Guid ComponentGuid => new("BC46DC07-C0BE-433F-9E2F-60CCBAA39148");

//    protected override dynamic Add(string url,Model<Type> type) => new Api().AddTypeToLocalKit(url, type);

//}

//public class AddDesignComponent : EngineComponent
//{
//    public AddDesignComponent()
//        : base("Add Design", "+Dsn",
//            "Add a design to a kit.",
//            "semio", "Loading/Saving")
//    {
//    }

//    public override Guid ComponentGuid => new("8B7AA946-0CB1-4CA8-A712-610B60425368");

//    protected override Bitmap Icon => Resources.design_add_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new DesignParam(), "Design", "Dn",
//            "Design to add to the kit.", GH_ParamAccess.item);
//        pManager.AddTextParameter("Directory", "Di?",
//            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
//            GH_ParamAccess.item);
//        pManager[1].Optional = true;
//        pManager.AddBooleanParameter("Run", "R", "Add the design to the kit.", GH_ParamAccess.item, false);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddBooleanParameter("Success", "Sc", "True if the design was added to the kit.",
//            GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var designGoo = new DesignGoo();
//        var path = "";
//        var run = false;

//        if (DA.GetData(0, ref designGoo))
//            designGoo = designGoo.Duplicate() as DesignGoo;
//        if (!DA.GetData(1, ref path))
//            path = OnPingDocument().IsFilePathDefined
//                ? Path.GetDirectoryName(OnPingDocument().FilePath)
//                : Directory.GetCurrentDirectory();
//        DA.GetData(2, ref run);

//        if (!run)
//        {
//            DA.SetData(0, false);
//            return;
//        }

//        var response = new Api().AddDesignToLocalKit(path, designGoo.Value);
//        if (response == null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
//            DA.SetData(0, false);
//            return;
//        }

//        if (response.Error != null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
//            DA.SetData(0, false);
//            return;
//        }

//        DA.SetData(0, true);
//    }
//}

//#endregion

//#region Removing

//public class RemoveTypeComponent : EngineComponent
//{
//    public RemoveTypeComponent()
//        : base("Remove Type", "-Typ",
//            "Remove a type from a kit.",
//            "semio", "Loading/Saving")
//    {
//    }

//    public override Guid ComponentGuid => new("F38D0E82-5A58-425A-B705-7A62FD9DB957");

//    protected override Bitmap Icon => Resources.type_remove_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Type Name", "TyNa",
//            "Name of the type to remove from the kit.", GH_ParamAccess.item);
//        pManager.AddTextParameter("Type Variant", "TyVn?",
//            "Optional variant of the type to remove from the kit. No variant will remove the default variant.",
//            GH_ParamAccess.item);
//        pManager[1].Optional = true;
//        pManager.AddTextParameter("Directory", "Di?",
//            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
//            GH_ParamAccess.item);
//        pManager[2].Optional = true;
//        pManager.AddBooleanParameter("Run", "R", "Remove the type from the kit.", GH_ParamAccess.item, false);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddBooleanParameter("Success", "Sc", "True if the type was removed from the kit.",
//            GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var typeName = "";
//        var typeVariant = "";
//        var path = "";
//        var run = false;

//        DA.GetData(0, ref typeName);
//        DA.GetData(1, ref typeVariant);
//        if (!DA.GetData(2, ref path))
//            path = OnPingDocument().IsFilePathDefined
//                ? Path.GetDirectoryName(OnPingDocument().FilePath)
//                : Directory.GetCurrentDirectory();
//        DA.GetData(3, ref run);

//        if (!run)
//        {
//            DA.SetData(0, false);
//            return;
//        }

//        var type = new TypeId
//        {
//            Name = typeName,
//            Variant = typeVariant
//        };
//        var response = new Api().RemoveTypeFromLocalKit(path, type);
//        if (response == null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
//            DA.SetData(0, false);
//            return;
//        }

//        if (response.Error != null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
//            DA.SetData(0, false);
//            return;
//        }

//        DA.SetData(0, true);
//    }
//}

//public class RemoveDesignComponent : EngineComponent
//{
//    public RemoveDesignComponent()
//        : base("Remove Design", "-Dsn",
//            "Remove a design from a kit.",
//            "semio", "Loading/Saving")
//    {
//    }

//    public override Guid ComponentGuid => new("9ECCE095-9D1E-4554-A3EB-1EAEEE2B12D5");

//    protected override Bitmap Icon => Resources.design_remove_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Design Name", "DnNa",
//            "Name of the design to remove from the kit.", GH_ParamAccess.item);
//        pManager.AddTextParameter("Design Variant", "DnVn?",
//            "Optional variant of the design to remove from the kit. No variant will remove the default variant.",
//            GH_ParamAccess.item);
//        pManager[1].Optional = true;
//        pManager.AddTextParameter("Directory", "Di?",
//            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
//            GH_ParamAccess.item);
//        pManager[2].Optional = true;
//        pManager.AddBooleanParameter("Run", "R", "Remove the design from the kit.", GH_ParamAccess.item, false);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddBooleanParameter("Success", "Sc", "True if the design was removed from the kit.",
//            GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var designName = "";
//        var designVariant = "";
//        var path = "";
//        var run = false;

//        DA.GetData(0, ref designName);
//        DA.GetData(1, ref designVariant);
//        if (!DA.GetData(2, ref path))
//            path = OnPingDocument().IsFilePathDefined
//                ? Path.GetDirectoryName(OnPingDocument().FilePath)
//                : Directory.GetCurrentDirectory();
//        DA.GetData(3, ref run);

//        if (!run)
//        {
//            DA.SetData(0, false);
//            return;
//        }

//        var design = new DesignId
//        {
//            Name = designName,
//            Variant = designVariant
//        };
//        var response = new Api().RemoveDesignFromLocalKit(path, design);
//        if (response == null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
//            DA.SetData(0, false);
//            return;
//        }

//        if (response.Error != null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
//            DA.SetData(0, false);
//            return;
//        }

//        DA.SetData(0, true);
//    }
//}

#endregion

#endregion

#region Scripting

//public class EncodeTextComponent : Component
//{
//    public EncodeTextComponent()
//        : base("Encode Text", ">Txt",
//            "Encode a text.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("FBDDF723-80BD-4AF9-A1EE-450A27D50ABE");

//    protected override Bitmap Icon => Resources.encode_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text to encode.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Encoded Text", "EnTx", "Encoded text.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var text = "";

//        DA.GetData(0, ref text);

//        var textBytes = Encoding.UTF8.GetBytes(text);
//        var base64Text = Convert.ToBase64String(textBytes);

//        DA.SetData(0, base64Text);
//    }
//}

//public class DecodeTextComponent : Component
//{
//    public DecodeTextComponent()
//        : base("Decode Text", "<Txt",
//            "Decode a text.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("E7158D28-87DE-493F-8D78-923265C3E211");

//    protected override Bitmap Icon => Resources.decode_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Encoded Text", "EnTx", "Encoded text to decode.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Decoded text.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var base64Text = "";

//        DA.GetData(0, ref base64Text);

//        var textBytes = Convert.FromBase64String(base64Text);
//        var text = Encoding.UTF8.GetString(textBytes);

//        DA.SetData(0, text);
//    }
//}

//#region Serialize

//public class SerializeQualityComponent : Component
//{
//    public SerializeQualityComponent()
//        : base("Serialize Quality", ">Qlt",
//            "Serialize a quality.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("C651F24C-BFF8-4821-8974-8588BCA75250");

//    protected override Bitmap Icon => Resources.quality_serialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
//            "Quality to serialize.", GH_ParamAccess.item);
//        pManager.AddBooleanParameter("Run", "R", "Serialize the quality.", GH_ParamAccess.item, false);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized quality.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var qualityGoo = new QualityGoo();

//        DA.GetData(0, ref qualityGoo);

//        var text = qualityGoo.Value.Serialize();
//        var textBytes = Encoding.UTF8.GetBytes(text);
//        var base64Text = Convert.ToBase64String(textBytes);

//        DA.SetData(0, base64Text);
//    }
//}

//public class SerializeTypeComponent : Component
//{
//    public SerializeTypeComponent()
//        : base("Serialize Type", ">Typ",
//            "Serialize a type.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("BD184BB8-8124-4604-835C-E7B7C199673A");

//    protected override Bitmap Icon => Resources.type_serialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new TypeParam(), "Type", "Ty",
//            "Type to serialize.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized type.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var typeGoo = new TypeGoo();

//        DA.GetData(0, ref typeGoo);
//        var text = typeGoo.Value.Serialize();
//        var textBytes = Encoding.UTF8.GetBytes(text);
//        var base64Text = Convert.ToBase64String(textBytes);

//        DA.SetData(0, base64Text);
//    }
//}

//public class SerializeDesignComponent : Component
//{
//    public SerializeDesignComponent()
//        : base("Serialize Design", ">For",
//            "Serialize a design.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("D755D6F1-27C4-441A-8856-6BA20E87DB58");

//    protected override Bitmap Icon => Resources.design_serialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new DesignParam(), "Design", "Dn",
//            "Design to serialize.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized design.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var designGoo = new DesignGoo();

//        DA.GetData(0, ref designGoo);
//        var text = designGoo.Value.Serialize();
//        var textBytes = Encoding.UTF8.GetBytes(text);
//        var base64Text = Convert.ToBase64String(textBytes);

//        DA.SetData(0, base64Text);
//    }
//}

//public class SerializeSceneComponent : Component
//{
//    public SerializeSceneComponent()
//        : base("Serialize Scene", ">Scn",
//            "Serialize a scene.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("2470CB4D-FC4A-4DCE-92BF-EDA281B36609");

//    protected override Bitmap Icon => Resources.scene_serialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
//            "Scene to serialize.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized scene.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var sceneGoo = new SceneGoo();

//        DA.GetData(0, ref sceneGoo);
//        var text = sceneGoo.Value.Serialize();
//        var textBytes = Encoding.UTF8.GetBytes(text);
//        var base64Text = Convert.ToBase64String(textBytes);

//        DA.SetData(0, base64Text);
//    }
//}

//#endregion

//#region Deserialize

//public class DeserializeQualityComponent : Component
//{
//    public DeserializeQualityComponent()
//        : base("Deserialize Quality", "<Qlt",
//            "Deserialize a quality.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("AECB1169-EB65-470F-966E-D491EB46A625");

//    protected override Bitmap Icon => Resources.quality_deserialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized quality.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new QualityParam(), "Quality", "Ql",
//            "Deserialized quality.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var base64Text = "";

//        DA.GetData(0, ref base64Text);

//        var textBytes = Convert.FromBase64String(base64Text);
//        var text = Encoding.UTF8.GetString(textBytes);

//        var quality = text.Deserialize<Quality>();

//        DA.SetData(0, new QualityGoo(quality));
//    }
//}

//public class DeserializeTypeComponent : Component
//{
//    public DeserializeTypeComponent()
//        : base("Deserialize Type", "<Typ",
//            "Deserialize a type.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("F21A80E0-2A62-4BFD-BC2B-A04363732F84");

//    protected override Bitmap Icon => Resources.type_deserialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized type.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new TypeParam(), "Type", "Ty",
//            "Deserialized type.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var base64Text = "";

//        DA.GetData(0, ref base64Text);
//        var textBytes = Convert.FromBase64String(base64Text);
//        var text = Encoding.UTF8.GetString(textBytes);

//        var type = text.Deserialize<Type>();

//        DA.SetData(0, new TypeGoo(type));
//    }
//}

//public class DeserializeDesignComponent : Component
//{
//    public DeserializeDesignComponent()
//        : base("Deserialize Design", "<For",
//            "Deserialize a design.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("464D4D72-CFF1-4391-8C31-9E37EB9434C6");

//    protected override Bitmap Icon => Resources.design_deserialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized design.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new DesignParam(), "Design", "Dn",
//            "Deserialized design.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var base64Text = "";

//        DA.GetData(0, ref base64Text);
//        var textBytes = Convert.FromBase64String(base64Text);
//        var text = Encoding.UTF8.GetString(textBytes);

//        var design = text.Deserialize<Design>();

//        DA.SetData(0, new DesignGoo(design));
//    }
//}

//public class DeserializeSceneComponent : Component
//{
//    public DeserializeSceneComponent()
//        : base("Deserialize Scene", "<Scn",
//            "Deserialize a scene.",
//            "semio", "Scripting")
//    {
//    }

//    public override Guid ComponentGuid => new("9A9AF239-6019-43E6-A3E1-59838BD5400B");

//    protected override Bitmap Icon => Resources.scene_deserialize_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Text", "Tx", "Text of serialized scene.", GH_ParamAccess.item);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
//            "Deserialized scene.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var base64Text = "";

//        DA.GetData(0, ref base64Text);
//        var textBytes = Convert.FromBase64String(base64Text);
//        var text = Encoding.UTF8.GetString(textBytes);

//        var scene = text.Deserialize<Scene>();

//        DA.SetData(0, new SceneGoo(scene));
//    }
//}

//#endregion

//#endregion

//#region Viewing

//public class GetSceneComponent : Component
//{
//    public GetSceneComponent()
//        : base("GetScene", "!Scn",
//            "Get a scene from a design.",
//            "semio", "Viewing")
//    {
//    }

//    public override Guid ComponentGuid => new("55F3BF32-3B4D-4355-BFAD-F3CA3847FC94");

//    protected override Bitmap Icon => Resources.scene_get_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddTextParameter("Design Name", "DnNa",
//            "Name of design to convert to a scene.", GH_ParamAccess.item);
//        pManager.AddTextParameter("Design Variant", "DnVn?",
//            "Optional variant of the design to convert to a scene. No variant will convert the default variant.",
//            GH_ParamAccess.item);
//        pManager[1].Optional = true;
//        pManager.AddTextParameter("Directory", "Di?",
//            "Optional directory path to the the kit. If none is provided, it will try to find if the Grasshopper script is executed inside a kit.",
//            GH_ParamAccess.item);
//        pManager[2].Optional = true;
//        pManager.AddBooleanParameter("Run", "R", "Convert the design to a scene.", GH_ParamAccess.item, false);
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new SceneParam(), "Scene", "Sc", "Scene.", GH_ParamAccess.item);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var designName = "";
//        var designVariant = "";
//        var path = "";
//        var run = false;

//        DA.GetData(0, ref designName);
//        DA.GetData(1, ref designVariant);
//        if (!DA.GetData(2, ref path))
//            path = OnPingDocument().IsFilePathDefined
//                ? Path.GetDirectoryName(OnPingDocument().FilePath)
//                : Directory.GetCurrentDirectory();
//        DA.GetData(3, ref run);

//        if (!run) return;

//        var response = new Api().DesignToSceneFromLocalKit(path, new DesignId
//        {
//            Name = designName,
//            Variant = designVariant
//        });
//        if (response == null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, Utility.ServerErrorMessage);
//            return;
//        }

//        if (response.Error != null)
//        {
//            AddRuntimeMessage(GH_RuntimeMessageLevel.Error, response.Error.Code + ": " + response.Error.Message);
//            return;
//        }

//        DA.SetData(0, new SceneGoo(response.Scene));
//    }
//}

//public class FilterSceneComponent : Component
//{
//    public FilterSceneComponent()
//        : base("FilterScene", "|Scn",
//            "Filter a scene.",
//            "semio", "Viewing")
//    {
//    }

//    public override Guid ComponentGuid => new("232796C0-5ADF-47FF-9FC4-058CB7003C5A");

//    protected override Bitmap Icon => Resources.scene_filter_24x24;

//    protected override void RegisterInputParams(GH_InputParamManager pManager)
//    {
//        pManager.AddParameter(new SceneParam(), "Scene", "Sc",
//            "Scene to filter.", GH_ParamAccess.item);
//        pManager.AddTextParameter("Level of Details", "LD*",
//            "Optional level of details of the representations in the scene.",
//            GH_ParamAccess.list);
//        pManager[1].Optional = true;
//        pManager.AddTextParameter("Tags", "Ta*", "Optional tags of the representations in the scene.",
//            GH_ParamAccess.list);
//        pManager[2].Optional = true;
//        pManager.AddTextParameter("Mimes", "Mm*", "Optional mimes of the representations in the scene.",
//            GH_ParamAccess.list);
//        pManager[3].Optional = true;
//    }

//    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
//    {
//        pManager.AddParameter(new RepresentationParam(), "Representations", "Rp+",
//            "Representation of the objects of the scene.", GH_ParamAccess.list);
//        pManager.AddPlaneParameter("Planes", "Pl+", "Planes of the objects of the scene.", GH_ParamAccess.list);
//        pManager.AddTextParameter("Pieces Ids", "PcId+",
//            "Ids of the pieces from the design that correspond to the objects of the scene.", GH_ParamAccess.list);
//        pManager.AddTextParameter("Parents Pieces Ids", "PaPcId+",
//            "Ids of the parent pieces from the design that correspond to the objects of the scene.",
//            GH_ParamAccess.list);
//    }

//    protected override void SolveInstance(IGH_DataAccess DA)
//    {
//        var sceneGoo = new SceneGoo();
//        var lods = new List<string>();
//        var tags = new List<string>();
//        var mimes = new List<string>();

//        DA.GetData(0, ref sceneGoo);
//        DA.GetDataList(1, lods);
//        DA.GetDataList(2, tags);
//        DA.GetDataList(3, mimes);

//        // filter the representations of the scene
//        // if lods are used, only the representations with the specified lods are returned
//        // if tags are used, each representations must have at least one of the specified tags
//        // if mimes are used, only the representations with the specified mimes are returned
//        var representations = sceneGoo.Value.Objects
//            .Select(o => o.Piece.Type.Representations
//                .First(r =>
//                {
//                    if (lods.Count > 0)
//                        if (!lods.Contains(r.Lod))
//                            return false;
//                    if (tags.Count > 0)
//                    {
//                        if (r.Tags == null)
//                            return false;
//                        if (!r.Tags.Any(t => tags.Contains(t)))
//                            return false;
//                    }

//                    if (mimes.Count > 0)
//                        if (!mimes.Contains(r.Mime))
//                            return false;
//                    return true;
//                })).ToList();
//        var planes = sceneGoo.Value.Objects.Select(o => o.Plane).ToList();
//        var piecesIds = sceneGoo.Value.Objects.Select(o => o.Piece.Id).ToList();
//        var parentsPiecesIds = sceneGoo.Value.Objects.Select(o => o.Parent?.Piece?.Id).ToList();

//        DA.SetDataList(0, representations.Select(r => new RepresentationGoo(r.DeepClone())));
//        DA.SetDataList(1, planes.Select(p => p.Convert()));
//        DA.SetDataList(2, piecesIds);
//        DA.SetDataList(3, parentsPiecesIds);
//    }
//}

//#endregion

#endregion

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