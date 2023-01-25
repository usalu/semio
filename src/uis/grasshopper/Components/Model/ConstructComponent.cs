//TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Components.Model
{
    public abstract class ConstructComponent : GH_Component
    {

        public ConstructComponent(string name, string nickname, string description, string category, string subCategory)
            : base(name, nickname, description, category, subCategory)
        {
        }
        //public abstract dynamic SemioType { get; }

        //protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        //{
        //    foreach (var field in SemioType.Descriptor.Fields.InFieldNumberOrder())
        //    {
        //        switch (field.FieldType)
        //        {
        //            case FieldType.String:
        //                pManager.AddTextParameter(field.Name, field.Name.First().ToString().ToUpper(), field.Declaration.LeadingComments, GH_ParamAccess.item);
        //                break;
        //            case FieldType.Bool:
        //                pManager.AddBooleanParameter(field.Name, field.Name.First().ToString().ToUpper(), field.Declaration.LeadingComments, GH_ParamAccess.item);
        //                break;
        //            default:
        //                pManager.AddParameter((IGH_Param)Activator.CreateInstance("Semio.UI.Grasshopper", field.Name + "Param").Unwrap());
        //                break;
        //        }
        //    }
        //}

        //protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        //{
        //    pManager.AddParameter((IGH_Param)Activator.CreateInstance("Semio.UI.Grasshopper", this.NickName + "Param").Unwrap());
        //}

        public override GH_Exposure Exposure => GH_Exposure.secondary;
    }
}


//Components could look like this:
//using System;
//using Semio.Model.V1;

//namespace Semio.UI.Grasshopper.Model
//{
//    public class ConstructDesignComponent : ConstructComponent
//    {
//        public override Type SemioType => typeof(Design);
//        public override Guid ComponentGuid => new Guid("712F6DA6-F74D-449F-83CE-956255450413");
//    }
//}