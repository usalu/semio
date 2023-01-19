using System;
using System.Collections.Generic;

using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class DeconstructElementComponent : DeconstructComponent
    {
        public DeconstructElementComponent()
          : base("Deconstruct Element", "DeElement", "Deconstruct an element", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_element;

        public override Guid ComponentGuid => new ("F7D844B8-495D-4A85-A748-4F400DC85183");

    }
}