using System;
using System.Collections.Generic;
using System.Runtime.Remoting.Messaging;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructParametersComponent : GH_Component
    {
        public ConstructParametersComponent()
          : base("Construct Parameters", "Parameters",
              "Description",
              "Semio", "Model")
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
        }

        ///// <summary>
        ///// Provides an Icon for the component.
        ///// </summary>
        //protected override System.Drawing.Bitmap Icon => Resources

        public override Guid ComponentGuid=>new Guid("94696A8F-8FF0-4CD0-919B-029252B68BCF");

        public override GH_Exposure Exposure => GH_Exposure.secondary;
    }
}