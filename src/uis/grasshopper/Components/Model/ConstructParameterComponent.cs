using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Xml.Linq;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructParameterComponent : ConstructComponent
    {
        public ConstructParameterComponent()
          : base("Construct Parameter", "Parameter", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Name", "N", "Name of the parameter", GH_ParamAccess.item);
            pManager.AddParameter(new ScopeParam(),"Context","Cx","",GH_ParamAccess.list);
            pManager[1].Optional = true;
            pManager.AddParameter(new ValueParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ParameterParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string name = "";
            if (!DA.GetData(0, ref name)) return;

            var context = new List<ScopeGoo>();
            DA.GetDataList(1, context);

            ValueGoo value = new();
            if (!DA.GetData(2, ref value)) return;

            Parameter parameter = new Parameter()
            {
                Name = name,
                Value = value.Value,
            };

            parameter.Context.AddRange(context.Select(x => x.Value));
          
            DA.SetData(0, new ParameterGoo(parameter));
        }
        public override Guid ComponentGuid=>new ("94696A8F-8FF0-4CD0-919B-029252B68BCF");
        protected override Bitmap Icon => Resources.icon_construct_parameter;
    }
}