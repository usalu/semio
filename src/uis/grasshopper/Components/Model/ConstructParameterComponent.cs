using System;
using System.Collections.Generic;
using System.Drawing;
using System.Runtime.Remoting.Messaging;
using Grasshopper.Kernel;
using Rhino.Geometry;
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
            pManager.AddTextParameter("Value", "V", "Value of the parameter", GH_ParamAccess.item);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ParameterParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string name = "";
            if (!DA.GetData(0, ref name)) return;

            string value = "";
            if (!DA.GetData(1, ref value)) return;

            DA.SetData(0, new ParameterGoo(new Parameter(name, value)));
        }
        public override Guid ComponentGuid=>new ("94696A8F-8FF0-4CD0-919B-029252B68BCF");
        protected override Bitmap Icon => Resources.icon_construct_parameter;
    }
}