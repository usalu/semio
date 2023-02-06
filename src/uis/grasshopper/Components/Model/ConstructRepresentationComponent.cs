using System;
using System.Collections.Generic;
using System.Drawing;
using Google.Protobuf;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructRepresentationComponent : ConstructComponent
    {
        public ConstructRepresentationComponent() : base("Construct Representation", "Representation", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Body", "B", "", GH_ParamAccess.item);
            pManager[0].Optional = true;
            pManager.AddParameter(new FileTypeParam());
            pManager[1].Optional = true;
            pManager.AddParameter(new PlatformParam());
            pManager[2].Optional = true;
            pManager.AddTextParameter("Description", "Dc", "", GH_ParamAccess.item);
            pManager[3].Optional = true;
            pManager.AddTextParameter("Concepts", "Cp", "", GH_ParamAccess.list);
            pManager[4].Optional = true;
            pManager.AddIntegerParameter("Level of Detail", "LD", "", GH_ParamAccess.item);
            pManager[5].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {

            string body = "";
            DA.GetData(0, ref body);

            FileTypeGoo fileType = new();
            DA.GetData(1, ref fileType);

            PlatformGoo platform = new();
            DA.GetData(2, ref platform);

            string description = "";
            DA.GetData(3, ref description);

            var concepts = new List<string>();
            DA.GetDataList(4, concepts);

            int lod = 0;
            DA.GetData(5, ref lod);


            Representation representation = new Representation()
            {
               Body = ByteString.CopyFromUtf8(body),
               FileType = fileType.Value,
               Platform = platform.Value,
               Description = description,
               Lod = lod
            };

            representation.Concepts.AddRange(concepts);

            DA.SetData(0, new RepresentationGoo(representation));
            }
        public override Guid ComponentGuid=> new("F1E3DA31-99F6-4261-B17E-141ACBE3CA2A");
        protected override Bitmap Icon => Resources.icon_construct_representation;
    }
}