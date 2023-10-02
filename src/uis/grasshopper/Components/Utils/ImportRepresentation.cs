using System;
using Grasshopper.Kernel;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;
using Semio.UI.Grasshopper.Utility;

namespace Semio.UI.Grasshopper.Components.Utils
{
    public class ImportRepresentationComponent : DeconstructComponent
    {
        public ImportRepresentationComponent()
          : base("Import Representation", "ImRepresentation", "", "semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddGenericParameter("Imported", "I", "", GH_ParamAccess.list);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            RepresentationGoo representation = new();
            if (!DA.GetData(0, ref representation)) return;

            DA.SetDataList(0, Converter.Convert(representation.Value));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_representation;

        public override Guid ComponentGuid => new ("7EA9685A-F484-4895-8D91-D513F7C53D14");

    }
}